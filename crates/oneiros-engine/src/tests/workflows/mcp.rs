//! MCP workflow — toolset-aware tool discovery and execution via the Model Context Protocol.
//!
//! MCP clients connect over streamable HTTP, initialize a session, discover
//! available tools, and call them. The server exposes a small root tool set
//! with toolset management. Activating a toolset expands the available tools.
//! Data created through MCP is visible through all other layers (HTTP REST, CLI).

use crate::tests::harness::TestApp;
use crate::*;

/// Helper: MCP-compliant POST request with required headers.
fn mcp_post(http: &reqwest::Client, url: &str) -> reqwest::RequestBuilder {
    http.post(url)
        .header("Accept", "application/json, text/event-stream")
        .header("Content-Type", "application/json")
}

/// Helper: extract JSON from an SSE response body.
/// MCP streamable HTTP wraps JSON-RPC in SSE `data:` lines.
/// The first `data:` line is a priming event (empty); we want the payload.
fn extract_sse_json(body: &str) -> serde_json::Value {
    let json_str = body
        .lines()
        .filter(|line| line.starts_with("data:"))
        .map(|line| line.strip_prefix("data:").unwrap().trim())
        .find(|s| !s.is_empty())
        .expect("response should contain a non-empty data: line");

    serde_json::from_str(json_str).expect("data: line should be valid JSON")
}

/// Helper: perform the MCP initialize handshake with a Bearer token, returning the session ID.
async fn mcp_initialize_with_token(
    http: &reqwest::Client,
    url: &str,
    token: &Token,
) -> Option<String> {
    let init_resp = mcp_post(http, url)
        .header("Authorization", format!("Bearer {token}"))
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "test-auth", "version": "0.1.0" }
            }
        }))
        .send()
        .await
        .unwrap();

    assert!(
        init_resp.status().is_success(),
        "MCP initialize should succeed"
    );

    let session_id = init_resp
        .headers()
        .get("mcp-session-id")
        .map(|v| v.to_str().unwrap().to_string());

    let mut notif = mcp_post(http, url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    }));
    if let Some(sid) = &session_id {
        notif = notif.header("mcp-session-id", sid);
    }
    notif.send().await.unwrap();

    session_id
}

/// Helper: perform the MCP initialize handshake, returning the session ID.
async fn mcp_initialize(http: &reqwest::Client, url: &str) -> Option<String> {
    let init_resp = mcp_post(http, url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "test", "version": "0.1.0" }
            }
        }))
        .send()
        .await
        .unwrap();

    assert!(
        init_resp.status().is_success(),
        "MCP initialize should succeed"
    );

    let session_id = init_resp
        .headers()
        .get("mcp-session-id")
        .map(|v| v.to_str().unwrap().to_string());

    // Send initialized notification
    let mut notif = mcp_post(http, url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    }));
    if let Some(sid) = &session_id {
        notif = notif.header("mcp-session-id", sid);
    }
    notif.send().await.unwrap();

    session_id
}

/// Helper: list tools for a session.
async fn mcp_list_tools(
    http: &reqwest::Client,
    url: &str,
    session_id: &Option<String>,
) -> Vec<String> {
    let mut req = mcp_post(http, url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 100,
        "method": "tools/list"
    }));
    if let Some(sid) = &session_id {
        req = req.header("mcp-session-id", sid);
    }
    let resp = req.send().await.unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().await.unwrap();
    let json = extract_sse_json(&body);
    json["result"]["tools"]
        .as_array()
        .expect("tools should be an array")
        .iter()
        .filter_map(|t| t["name"].as_str().map(String::from))
        .collect()
}

/// Helper: call an MCP tool and return the result JSON.
async fn mcp_call_tool(
    http: &reqwest::Client,
    url: &str,
    session_id: &Option<String>,
    id: u64,
    tool_name: &str,
    arguments: serde_json::Value,
) -> serde_json::Value {
    let mut req = mcp_post(http, url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": arguments
        }
    }));
    if let Some(sid) = session_id {
        req = req.header("mcp-session-id", sid);
    }
    let resp = req.send().await.unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().await.unwrap();
    extract_sse_json(&body)
}

/// Helper: send an MCP JSON-RPC request and return the result JSON.
async fn mcp_request(
    http: &reqwest::Client,
    url: &str,
    session_id: &Option<String>,
    id: u64,
    method: &str,
    params: Option<serde_json::Value>,
) -> serde_json::Value {
    let mut body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
    });
    if let Some(p) = params {
        body["params"] = p;
    }
    let mut req = mcp_post(http, url).json(&body);
    if let Some(sid) = session_id {
        req = req.header("mcp-session-id", sid);
    }
    let resp = req.send().await.unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().await.unwrap();
    extract_sse_json(&body)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn mcp_session() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let http = reqwest::Client::new();
    let url = app.mcp_url();

    // Initialize a session
    let session_id = mcp_initialize(&http, &url).await;

    // ── Root tools are small ───────────────────────────────────────
    let root_tools = mcp_list_tools(&http, &url, &session_id).await;
    assert!(
        root_tools.len() <= 10,
        "root should have few tools, got {}",
        root_tools.len()
    );
    assert!(root_tools.contains(&"activate-toolset".to_string()));
    assert!(root_tools.contains(&"deactivate-toolset".to_string()));
    assert!(root_tools.contains(&"get-pressure".to_string()));

    // Domain tools should NOT be in root
    assert!(!root_tools.contains(&"create-agent".to_string()));
    assert!(!root_tools.contains(&"add-cognition".to_string()));

    // ── Activate admin toolset ─────────────────────────────────────
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        2,
        "activate-toolset",
        serde_json::json!({ "toolset": "admin" }),
    )
    .await;
    let is_error = result["result"]["isError"].as_bool().unwrap_or(false);
    assert!(!is_error, "activate-toolset should succeed: {result}");

    // ── Tools list now includes admin tools ─────────────────────────
    let admin_tools = mcp_list_tools(&http, &url, &session_id).await;
    assert!(
        admin_tools.len() > root_tools.len(),
        "admin toolset should add tools (root: {}, admin: {})",
        root_tools.len(),
        admin_tools.len()
    );
    assert!(admin_tools.contains(&"create-agent".to_string()));
    assert!(admin_tools.contains(&"set-level".to_string()));
    assert!(admin_tools.contains(&"set-persona".to_string()));
    // Root tools are still present
    assert!(admin_tools.contains(&"activate-toolset".to_string()));

    // Tool names should be unique
    let mut seen = std::collections::HashSet::new();
    for name in &admin_tools {
        assert!(seen.insert(name), "duplicate tool name: {name}");
    }

    // Every tool should have a name and inputSchema (spot check via raw request)
    let mut tools_req = mcp_post(&http, &url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 20,
        "method": "tools/list"
    }));
    if let Some(sid) = &session_id {
        tools_req = tools_req.header("mcp-session-id", sid);
    }
    let tools_resp = tools_req.send().await?;
    let body = tools_resp.text().await?;
    let json = extract_sse_json(&body);
    for tool in json["result"]["tools"].as_array().unwrap() {
        assert!(tool["name"].is_string(), "tool should have a name");
        assert!(
            tool["inputSchema"].is_object(),
            "tool should have an inputSchema"
        );
    }

    // ── Call a tool through the toolset ─────────────────────────────
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "set-persona",
        serde_json::json!({
            "name": "mcp-persona",
            "description": "Created via MCP",
            "prompt": "You were born from a tool call"
        }),
    )
    .await;
    let is_error = result["result"]["isError"].as_bool().unwrap_or(false);
    assert!(!is_error, "set-persona should succeed: {result}");

    // Data created through MCP is visible through the HTTP REST API
    match app
        .client()
        .persona()
        .get(&PersonaName::new("mcp-persona"))
        .await?
    {
        PersonaResponse::PersonaDetails(p) => {
            assert_eq!(p.data.description.to_string(), "Created via MCP");
        }
        other => panic!("expected PersonaDetails, got {other:?}"),
    }

    // And through the CLI
    let cmd_result = app.command("persona show mcp-persona").await?;
    let cmd_json = serde_json::to_value(cmd_result.response())?;
    assert_eq!(cmd_json["data"]["name"], "mcp-persona");

    // ── Deactivate returns to root ──────────────────────────────────
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        4,
        "deactivate-toolset",
        serde_json::json!({}),
    )
    .await;
    let is_error = result["result"]["isError"].as_bool().unwrap_or(false);
    assert!(!is_error, "deactivate-toolset should succeed: {result}");

    let deactivated_tools = mcp_list_tools(&http, &url, &session_id).await;
    assert_eq!(
        deactivated_tools.len(),
        root_tools.len(),
        "should return to root tool count"
    );

    // ── Switch to capture toolset ───────────────────────────────────
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        5,
        "activate-toolset",
        serde_json::json!({ "toolset": "capture" }),
    )
    .await;
    let is_error = result["result"]["isError"].as_bool().unwrap_or(false);
    assert!(!is_error, "activate capture should succeed: {result}");

    let capture_tools = mcp_list_tools(&http, &url, &session_id).await;
    assert!(capture_tools.contains(&"add-cognition".to_string()));
    assert!(capture_tools.contains(&"search-query".to_string()));
    // Admin tools should be gone
    assert!(!capture_tools.contains(&"create-agent".to_string()));

    Ok(())
}

#[tokio::test]
async fn mcp_session_auth() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let token = app.token().expect("project should have a token");
    let http = reqwest::Client::new();
    let url = app.mcp_url();

    // Initialize with Bearer token
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    // Activate admin toolset to access persona tools
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        2,
        "activate-toolset",
        serde_json::json!({ "toolset": "admin" }),
    )
    .await;
    let is_error = result["result"]["isError"].as_bool().unwrap_or(false);
    assert!(!is_error, "activate-toolset should succeed: {result}");

    // Create a persona through the authenticated session
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "set-persona",
        serde_json::json!({
            "name": "auth-persona",
            "description": "Created via authenticated MCP",
            "prompt": "You were born from a token"
        }),
    )
    .await;

    // Should succeed (not isError)
    let is_error = result["result"]["isError"].as_bool().unwrap_or(false);
    assert!(
        !is_error,
        "authenticated tool call should succeed, got: {result}"
    );

    // Data should be visible through the HTTP REST API (same brain)
    match app
        .client()
        .persona()
        .get(&PersonaName::new("auth-persona"))
        .await?
    {
        PersonaResponse::PersonaDetails(p) => {
            assert_eq!(
                p.data.description.to_string(),
                "Created via authenticated MCP"
            );
        }
        other => panic!("expected PersonaDetails, got {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn mcp_error_paths() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let http = reqwest::Client::new();
    let url = app.mcp_url();
    let session_id = mcp_initialize(&http, &url).await;

    // ── Unknown tool ────────────────────────────────────────────

    let mut req = mcp_post(&http, &url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 10,
        "method": "tools/call",
        "params": {
            "name": "nonexistent_tool",
            "arguments": {}
        }
    }));
    if let Some(sid) = &session_id {
        req = req.header("mcp-session-id", sid);
    }
    let resp = req.send().await?;

    // MCP returns success at HTTP level but with isError in the result
    assert!(resp.status().is_success());
    let body = resp.text().await?;
    let json = extract_sse_json(&body);

    // The result should indicate an error
    let is_error = json["result"]["isError"].as_bool().unwrap_or(false);
    assert!(
        is_error,
        "unknown tool should return isError=true, got: {json}"
    );

    // ── Invalid toolset name ───────────────────────────────────────

    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        11,
        "activate-toolset",
        serde_json::json!({ "toolset": "nonexistent" }),
    )
    .await;
    // activate-toolset with bad name returns a JSON-RPC error (invalid params)
    let has_error = result.get("error").is_some();
    assert!(
        has_error,
        "bad toolset name should return error, got: {result}"
    );

    // ── Activate admin, then test domain errors ────────────────────

    mcp_call_tool(
        &http,
        &url,
        &session_id,
        12,
        "activate-toolset",
        serde_json::json!({ "toolset": "admin" }),
    )
    .await;

    // Bad parameters
    let mut req = mcp_post(&http, &url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 13,
        "method": "tools/call",
        "params": {
            "name": "get-agent",
            "arguments": { "wrong_field": "not a name" }
        }
    }));
    if let Some(sid) = &session_id {
        req = req.header("mcp-session-id", sid);
    }
    let resp = req.send().await?;
    assert!(resp.status().is_success());
    let body = resp.text().await?;
    let json = extract_sse_json(&body);
    let is_error = json["result"]["isError"].as_bool().unwrap_or(false);
    assert!(
        is_error,
        "bad params should return isError=true, got: {json}"
    );

    // Domain error — nonexistent agent
    let mut req = mcp_post(&http, &url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 14,
        "method": "tools/call",
        "params": {
            "name": "get-agent",
            "arguments": { "name": "ghost.nobody" }
        }
    }));
    if let Some(sid) = &session_id {
        req = req.header("mcp-session-id", sid);
    }
    let resp = req.send().await?;
    assert!(resp.status().is_success());
    let body = resp.text().await?;
    let json = extract_sse_json(&body);
    let is_error = json["result"]["isError"].as_bool().unwrap_or(false);
    assert!(
        is_error,
        "nonexistent agent via MCP should return isError=true, got: {json}"
    );

    Ok(())
}

#[tokio::test]
async fn mcp_resource_templates() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let http = reqwest::Client::new();
    let url = app.mcp_url();
    let session_id = mcp_initialize(&http, &url).await;

    // ── List resource templates ────────────────────────────────────

    let json = mcp_request(&http, &url, &session_id, 2, "resources/templates/list", None).await;
    let templates = json["result"]["resourceTemplates"]
        .as_array()
        .expect("resourceTemplates should be an array");

    assert!(
        templates.len() >= 3,
        "should have at least 3 resource templates, got {}",
        templates.len()
    );

    let template_uris: Vec<&str> = templates
        .iter()
        .filter_map(|t| t["uriTemplate"].as_str())
        .collect();
    assert!(template_uris.contains(&"oneiros://agent/{name}/dream"));
    assert!(template_uris.contains(&"oneiros://agent/{name}/pressure"));
    assert!(template_uris.contains(&"oneiros://agent/{name}/guidebook"));

    // Every template should have a name and description
    for template in templates {
        assert!(template["name"].is_string(), "template should have a name");
        assert!(
            template["description"].is_string(),
            "template should have a description"
        );
    }

    Ok(())
}

#[tokio::test]
async fn mcp_read_resource() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let http = reqwest::Client::new();
    let url = app.mcp_url();
    let session_id = mcp_initialize(&http, &url).await;

    // seed_core creates vocabulary but not agents — seed agents too
    let _ = app.command("seed agents").await?;

    // ── Read pressure resource ──────────────────────────────────────

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        10,
        "resources/read",
        Some(serde_json::json!({ "uri": "oneiros://agent/governor.process/pressure" })),
    )
    .await;

    // Pressure may error if agent not found — check either way
    if let Some(err) = json.get("error") {
        panic!("pressure resource returned error: {err}");
    }
    let contents = json["result"]["contents"]
        .as_array()
        .unwrap_or_else(|| panic!("pressure contents should be an array, got: {json}"));
    assert!(!contents.is_empty(), "pressure resource should return content");
    assert!(
        contents[0]["text"].is_string(),
        "resource content should have text"
    );

    // ── Read dream resource ─────────────────────────────────────────

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        11,
        "resources/read",
        Some(serde_json::json!({ "uri": "oneiros://agent/governor.process/dream" })),
    )
    .await;

    let contents = json["result"]["contents"]
        .as_array()
        .unwrap_or_else(|| panic!("dream contents should be an array, got: {json}"));
    assert!(!contents.is_empty(), "dream resource should return content");
    let text = contents[0]["text"].as_str().unwrap_or("");
    assert!(
        text.contains("governor.process"),
        "dream should mention the agent name, got: {text}"
    );

    // ── Read guidebook resource ─────────────────────────────────────

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        12,
        "resources/read",
        Some(serde_json::json!({ "uri": "oneiros://agent/governor.process/guidebook" })),
    )
    .await;

    let contents = json["result"]["contents"]
        .as_array()
        .unwrap_or_else(|| panic!("guidebook contents should be an array, got: {json}"));
    assert!(
        !contents.is_empty(),
        "guidebook resource should return content"
    );

    // ── Unknown resource kind returns error ──────────────────────────

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        13,
        "resources/read",
        Some(serde_json::json!({ "uri": "oneiros://agent/governor.process/bogus" })),
    )
    .await;
    assert!(
        json.get("error").is_some(),
        "unknown resource kind should return error, got: {json}"
    );

    Ok(())
}

#[tokio::test]
async fn mcp_prompts() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let http = reqwest::Client::new();
    let url = app.mcp_url();
    let session_id = mcp_initialize(&http, &url).await;

    // ── List prompts ────────────────────────────────────────────────

    let json = mcp_request(&http, &url, &session_id, 2, "prompts/list", None).await;
    let prompts = json["result"]["prompts"]
        .as_array()
        .expect("prompts should be an array");

    let prompt_names: Vec<&str> = prompts
        .iter()
        .filter_map(|p| p["name"].as_str())
        .collect();
    assert!(prompt_names.contains(&"orient"), "should have orient prompt");
    assert!(
        prompt_names.contains(&"toolsets"),
        "should have toolsets prompt"
    );

    // Every prompt should have a description
    for prompt in prompts {
        assert!(
            prompt["description"].is_string(),
            "prompt should have a description"
        );
    }

    // Orient prompt should declare an agent argument
    let orient = prompts
        .iter()
        .find(|p| p["name"] == "orient")
        .expect("orient prompt should exist");
    let args = orient["arguments"]
        .as_array()
        .expect("orient should have arguments");
    assert_eq!(args[0]["name"], "agent");
    assert_eq!(args[0]["required"], true);

    // ── Get toolsets prompt ──────────────────────────────────────────

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        3,
        "prompts/get",
        Some(serde_json::json!({ "name": "toolsets" })),
    )
    .await;

    let messages = json["result"]["messages"]
        .as_array()
        .expect("messages should be an array");
    assert!(!messages.is_empty(), "toolsets prompt should return messages");

    let text = messages[0]["content"]["text"].as_str().unwrap_or("");
    assert!(text.contains("lifecycle"), "toolsets should list lifecycle");
    assert!(text.contains("capture"), "toolsets should list capture");
    assert!(text.contains("garden"), "toolsets should list garden");
    assert!(text.contains("admin"), "toolsets should list admin");
    assert!(text.contains("distribute"), "toolsets should list distribute");

    // ── Get orient prompt with agent ────────────────────────────────

    // Create agent through MCP
    mcp_call_tool(
        &http, &url, &session_id, 10,
        "activate-toolset",
        serde_json::json!({ "toolset": "admin" }),
    ).await;
    mcp_call_tool(
        &http, &url, &session_id, 11,
        "create-agent",
        serde_json::json!({
            "name": "test.agent",
            "persona": "process",
            "description": "Test agent for orient prompt",
            "prompt": ""
        }),
    ).await;
    mcp_call_tool(
        &http, &url, &session_id, 12,
        "deactivate-toolset",
        serde_json::json!({}),
    ).await;

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        4,
        "prompts/get",
        Some(serde_json::json!({
            "name": "orient",
            "arguments": { "agent": "test.agent" }
        })),
    )
    .await;

    let messages = json["result"]["messages"]
        .as_array()
        .expect("messages should be an array");
    assert!(!messages.is_empty(), "orient prompt should return messages");

    let text = messages[0]["content"]["text"].as_str().unwrap_or("");
    assert!(
        text.contains("test.agent"),
        "orient should reference the agent, got: {text}"
    );
    assert!(
        text.contains("activate-toolset"),
        "orient should suggest activating a toolset, got: {text}"
    );

    // ── Orient without agent argument returns error ──────────────────

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        5,
        "prompts/get",
        Some(serde_json::json!({ "name": "orient" })),
    )
    .await;
    assert!(
        json.get("error").is_some(),
        "orient without agent should return error, got: {json}"
    );

    Ok(())
}
