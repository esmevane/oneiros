//! MCP workflow — tool discovery and execution via the Model Context Protocol.
//!
//! MCP clients connect over streamable HTTP, initialize a session, discover
//! available tools, and call them. Data created through MCP is visible
//! through all other layers (HTTP REST, CLI).
//!
//! Tests cover: session basics, toolsets, resources, response format,
//! prompts, cross-layer visibility, error paths, and auth.

use crate::tests::harness::TestApp;
use crate::*;

// ── Helpers ─────────────────────────────────────────────────────

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

/// Helper: perform the MCP initialize handshake, returning the session ID.
/// Also verifies the server advertises tools, resources, and prompts capabilities.
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

    // Verify server advertises all three primitives
    let body = init_resp.text().await.unwrap();
    let json = extract_sse_json(&body);
    let caps = &json["result"]["capabilities"];
    assert!(caps["tools"].is_object(), "should advertise tools");
    assert!(caps["resources"].is_object(), "should advertise resources");
    assert!(caps["prompts"].is_object(), "should advertise prompts");

    // Send initialized notification
    let mut notif = mcp_post(http, url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    }));
    if let Some(ref sid) = session_id {
        notif = notif.header("mcp-session-id", sid);
    }
    notif.send().await.unwrap();

    session_id
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
    if let Some(ref sid) = session_id {
        notif = notif.header("mcp-session-id", sid);
    }
    notif.send().await.unwrap();

    session_id
}

/// Helper: call an MCP method and return the result JSON.
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

/// Helper: call an MCP tool and return the result JSON.
async fn mcp_call_tool(
    http: &reqwest::Client,
    url: &str,
    session_id: &Option<String>,
    id: u64,
    tool_name: &str,
    arguments: serde_json::Value,
) -> serde_json::Value {
    mcp_request(
        http,
        url,
        session_id,
        id,
        "tools/call",
        Some(serde_json::json!({
            "name": tool_name,
            "arguments": arguments
        })),
    )
    .await
}

/// Helper: extract text content from a tool call result.
fn result_text(json: &serde_json::Value) -> String {
    json["result"]["content"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|c| c["text"].as_str())
        .unwrap_or("")
        .to_string()
}

/// Helper: collect tool names from a tools/list result.
fn tool_names(json: &serde_json::Value) -> Vec<String> {
    json["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|t| t["name"].as_str().map(String::from))
        .collect()
}

// ── Session basics ───────────────────────────────────────────────

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

    // Discover tools — root layer only on fresh session
    let json = mcp_request(&http, &url, &session_id, 2, "tools/list", None).await;
    let tools = json["result"]["tools"]
        .as_array()
        .expect("tools should be an array");

    // Root tools only — activate-toolset, deactivate-toolset, pressure tools
    let tool_names_vec: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
    assert!(
        tool_names_vec.contains(&"activate-toolset"),
        "root should include activate-toolset"
    );

    // Every tool should have a name and inputSchema
    for tool in tools {
        assert!(tool["name"].is_string(), "tool should have a name");
        assert!(
            tool["inputSchema"].is_object(),
            "tool should have an inputSchema"
        );
    }

    // Tool names should be unique
    let mut seen = std::collections::HashSet::new();
    for name in &tool_names_vec {
        assert!(seen.insert(name), "duplicate tool name: {name}");
    }

    // Activate vocabulary toolset
    let activate_result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "activate-toolset",
        serde_json::json!({ "name": "vocabulary" }),
    )
    .await;
    assert!(
        !activate_result["result"]["isError"]
            .as_bool()
            .unwrap_or(false),
        "activate-toolset should succeed"
    );

    // Call a tool — create a persona through MCP
    mcp_call_tool(
        &http,
        &url,
        &session_id,
        4,
        "set-persona",
        serde_json::json!({
            "name": "mcp-persona",
            "description": "Created via MCP",
            "prompt": "You were born from a tool call"
        }),
    )
    .await;

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

    Ok(())
}

// ── Auth ─────────────────────────────────────────────────────────

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

    // Initialize with Bearer token — activate vocabulary toolset first
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    mcp_call_tool(
        &http,
        &url,
        &session_id,
        2,
        "activate-toolset",
        serde_json::json!({ "name": "vocabulary" }),
    )
    .await;

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

// ── Toolsets ─────────────────────────────────────────────────────

/// On connect, only root tools are visible — not the full 89-tool catalog.
#[tokio::test]
async fn tools_scoped_to_root_on_connect() -> Result<(), Box<dyn core::error::Error>> {
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

    let json = mcp_request(&http, &url, &session_id, 2, "tools/list", None).await;
    let names = tool_names(&json);

    // Root layer: activate-toolset, deactivate-toolset, and pressure tools
    assert!(
        names.len() <= 10,
        "root should have at most 10 tools, got {}: {names:?}",
        names.len()
    );
    assert!(names.contains(&"activate-toolset".to_string()));
    assert!(names.contains(&"deactivate-toolset".to_string()));

    Ok(())
}

/// Activating a toolset expands the tool list to root + that toolset.
#[tokio::test]
async fn toolset_activation_expands_tools() -> Result<(), Box<dyn core::error::Error>> {
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

    // Activate the continuity toolset
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "activate-toolset",
        serde_json::json!({ "name": "continuity" }),
    )
    .await;
    assert!(
        !result["result"]["isError"].as_bool().unwrap_or(false),
        "activate-toolset should succeed"
    );

    // Now tools/list should include continuity tools
    let json = mcp_request(&http, &url, &session_id, 4, "tools/list", None).await;
    let names = tool_names(&json);

    assert!(names.contains(&"add-cognition".to_string()));
    assert!(names.contains(&"add-memory".to_string()));
    assert!(names.contains(&"search-query".to_string()));
    // Root tools should still be present
    assert!(names.contains(&"activate-toolset".to_string()));
    // Lifecycle tools should NOT be present (different toolset)
    assert!(!names.contains(&"wake-agent".to_string()));
    assert!(!names.contains(&"dream-agent".to_string()));

    // Total should be within budget
    assert!(
        names.len() <= 16,
        "root + continuity should be at most 16 tools, got {}: {names:?}",
        names.len()
    );

    Ok(())
}

/// Deactivating returns to root-only tools.
#[tokio::test]
async fn toolset_deactivation_returns_to_root() -> Result<(), Box<dyn core::error::Error>> {
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

    // Activate then deactivate
    mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "activate-toolset",
        serde_json::json!({ "name": "lifecycle" }),
    )
    .await;

    mcp_call_tool(
        &http,
        &url,
        &session_id,
        4,
        "deactivate-toolset",
        serde_json::json!({}),
    )
    .await;

    let json = mcp_request(&http, &url, &session_id, 5, "tools/list", None).await;
    let names = tool_names(&json);

    assert!(
        names.len() <= 10,
        "after deactivation should be back to root, got {}: {names:?}",
        names.len()
    );
    assert!(!names.contains(&"wake-agent".to_string()));

    Ok(())
}

// ── Resources ────────────────────────────────────────────────────

/// Static resources are listed without parameters.
#[tokio::test]
async fn resources_list_static() -> Result<(), Box<dyn core::error::Error>> {
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

    let json = mcp_request(&http, &url, &session_id, 2, "resources/list", None).await;
    let resources = json["result"]["resources"]
        .as_array()
        .expect("resources should be an array");

    let uris: Vec<&str> = resources.iter().filter_map(|r| r["uri"].as_str()).collect();

    assert!(uris.contains(&"oneiros-mcp://vocabulary"));
    assert!(uris.contains(&"oneiros-mcp://agents"));
    assert!(uris.contains(&"oneiros-mcp://bookmarks"));
    assert!(uris.contains(&"oneiros-mcp://storage"));

    Ok(())
}

/// Resource templates are listed for parametric resources.
#[tokio::test]
async fn resources_list_templates() -> Result<(), Box<dyn core::error::Error>> {
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

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        2,
        "resources/templates/list",
        None,
    )
    .await;
    let templates = json["result"]["resourceTemplates"]
        .as_array()
        .expect("resourceTemplates should be an array");

    let uri_templates: Vec<&str> = templates
        .iter()
        .filter_map(|t| t["uriTemplate"].as_str())
        .collect();

    assert!(uri_templates.contains(&"oneiros-mcp://agent/{name}/status"));
    assert!(uri_templates.contains(&"oneiros-mcp://agent/{name}/dream"));
    assert!(uri_templates.contains(&"oneiros-mcp://agent/{name}/pressure"));
    assert!(uri_templates.contains(&"oneiros-mcp://agent/{name}/cognitions"));
    assert!(uri_templates.contains(&"oneiros-mcp://agent/{name}/memories"));
    assert!(uri_templates.contains(&"oneiros-mcp://record/{ref}"));

    Ok(())
}

/// Reading a resource returns authored markdown, not raw JSON.
#[tokio::test]
async fn resource_read_returns_markdown() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    // Create an agent so the resource has content
    app.command("agent create navigator process").await?;

    let http = reqwest::Client::new();
    let url = app.mcp_url();
    let session_id = mcp_initialize(&http, &url).await;

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        2,
        "resources/read",
        Some(serde_json::json!({ "uri": "oneiros-mcp://agents" })),
    )
    .await;

    let contents = json["result"]["contents"]
        .as_array()
        .expect("contents should be an array");

    assert!(!contents.is_empty(), "should have content");
    let text = contents[0]["text"]
        .as_str()
        .expect("should be text content");

    // Should be authored markdown, not JSON
    assert!(
        !text.starts_with('{') && !text.starts_with('['),
        "resource should return markdown, not JSON: {text}"
    );
    assert!(
        text.contains("navigator"),
        "should include the created agent"
    );

    Ok(())
}

// ── Tool consolidation ───────────────────────────────────────────

/// Vocabulary tools work through the vocabulary toolset.
#[tokio::test]
async fn vocabulary_toolset_manages_domains() -> Result<(), Box<dyn core::error::Error>> {
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

    // Activate vocabulary toolset
    mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "activate-toolset",
        serde_json::json!({ "name": "vocabulary" }),
    )
    .await;

    // Create a persona through its domain tool
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        4,
        "set-persona",
        serde_json::json!({
            "name": "mcp-persona",
            "description": "Created via vocabulary toolset",
            "prompt": "You were born from a domain tool"
        }),
    )
    .await;

    assert!(
        !result["result"]["isError"].as_bool().unwrap_or(false),
        "set-persona should succeed: {result}"
    );

    // Should be visible through REST
    match app
        .client()
        .persona()
        .get(&PersonaName::new("mcp-persona"))
        .await?
    {
        PersonaResponse::PersonaDetails(p) => {
            assert_eq!(
                p.data.description.to_string(),
                "Created via vocabulary toolset"
            );
        }
        other => panic!("expected PersonaDetails, got {other:?}"),
    }

    // Also works for other vocabulary kinds
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        5,
        "set-texture",
        serde_json::json!({
            "name": "mcp-texture",
            "description": "A texture from MCP",
            "prompt": ""
        }),
    )
    .await;

    assert!(
        !result["result"]["isError"].as_bool().unwrap_or(false),
        "set-texture should succeed: {result}"
    );

    Ok(())
}

// ── Response format ──────────────────────────────────────────────

/// Tool responses are authored markdown with a hints section.
#[tokio::test]
async fn tool_responses_are_markdown_with_hints() -> Result<(), Box<dyn core::error::Error>> {
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

    // Create an agent through MCP first
    let create_result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "create-agent",
        serde_json::json!({
            "name": "thinker",
            "persona": "process",
            "description": "",
            "prompt": ""
        }),
    )
    .await;
    let create_is_error = create_result["result"]["isError"]
        .as_bool()
        .unwrap_or(false);
    let create_text = result_text(&create_result);
    assert!(
        !create_is_error,
        "create-agent should succeed: {create_text}"
    );
    assert!(
        create_text.contains("thinker"),
        "create response should mention agent name: {create_text}"
    );

    // Call add-cognition (agent name is normalized to name.persona)
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        4,
        "add-cognition",
        serde_json::json!({
            "agent": "thinker.process",
            "texture": "observation",
            "content": "The MCP redesign is taking shape"
        }),
    )
    .await;

    assert!(
        !result["result"]["isError"].as_bool().unwrap_or(false),
        "add-cognition should succeed: {result}"
    );

    // Response should be text, not JSON
    let content = result["result"]["content"]
        .as_array()
        .expect("content should be an array");
    let first = &content[0];
    assert_eq!(
        first["type"].as_str(),
        Some("text"),
        "response should be text content, not json"
    );

    let text = first["text"].as_str().unwrap();

    // Should contain a hints section
    assert!(
        text.contains("## Hints"),
        "response should include a hints section: {text}"
    );
    // Hints should have the structured format
    assert!(
        text.contains("**suggest**")
            || text.contains("**inspect**")
            || text.contains("**follow-up**"),
        "hints should have leveled format: {text}"
    );

    Ok(())
}

/// Error responses include recovery hints.
#[tokio::test]
async fn error_responses_include_hints() -> Result<(), Box<dyn core::error::Error>> {
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

    // Call a tool that will fail — nonexistent agent
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "add-cognition",
        serde_json::json!({
            "agent": "ghost.nobody",
            "texture": "observation",
            "content": "This should fail"
        }),
    )
    .await;

    let is_error = result["result"]["isError"].as_bool().unwrap_or(false);
    assert!(is_error, "should be an error result");

    let text = result_text(&result);
    assert!(
        text.contains("## Hints"),
        "error should include recovery hints: {text}"
    );

    Ok(())
}

// ── Prompts ──────────────────────────────────────────────────────

/// Prompts are listed and can be retrieved.
#[tokio::test]
async fn prompts_orient_and_toolsets() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    // Create an agent for the orient prompt
    app.command("agent create navigator process").await?;

    let http = reqwest::Client::new();
    let url = app.mcp_url();
    let session_id = mcp_initialize(&http, &url).await;

    // List prompts
    let json = mcp_request(&http, &url, &session_id, 2, "prompts/list", None).await;
    let prompts = json["result"]["prompts"]
        .as_array()
        .expect("prompts should be an array");

    let names: Vec<&str> = prompts.iter().filter_map(|p| p["name"].as_str()).collect();

    assert!(names.contains(&"orient"), "should have orient prompt");
    assert!(names.contains(&"toolsets"), "should have toolsets prompt");

    // Get the orient prompt
    let json = mcp_request(
        &http,
        &url,
        &session_id,
        3,
        "prompts/get",
        Some(serde_json::json!({
            "name": "orient",
            "arguments": { "agent": "navigator" }
        })),
    )
    .await;

    let messages = json["result"]["messages"]
        .as_array()
        .expect("orient should return messages");
    assert!(!messages.is_empty(), "orient should have content");

    // Get the toolsets prompt (no arguments)
    let json = mcp_request(
        &http,
        &url,
        &session_id,
        4,
        "prompts/get",
        Some(serde_json::json!({
            "name": "toolsets"
        })),
    )
    .await;

    let messages = json["result"]["messages"]
        .as_array()
        .expect("toolsets should return messages");
    assert!(!messages.is_empty(), "toolsets should have content");

    // Toolsets prompt should mention the available toolsets
    let text = messages[0]["content"]["text"].as_str().unwrap_or_default();
    assert!(
        text.contains("lifecycle"),
        "should mention lifecycle toolset"
    );
    assert!(
        text.contains("continuity"),
        "should mention continuity toolset"
    );
    assert!(
        text.contains("vocabulary"),
        "should mention vocabulary toolset"
    );

    Ok(())
}

// ── Cross-layer visibility ───────────────────────────────────────

/// The MCP layer maintains cross-layer data visibility with CLI and REST.
#[tokio::test]
async fn cross_layer_visibility() -> Result<(), Box<dyn core::error::Error>> {
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

    // Create agent through MCP
    let create_result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "create-agent",
        serde_json::json!({
            "name": "witness",
            "persona": "process",
            "description": "",
            "prompt": ""
        }),
    )
    .await;
    assert!(
        !create_result["result"]["isError"]
            .as_bool()
            .unwrap_or(false),
        "create-agent should succeed: {}",
        result_text(&create_result)
    );

    // Add cognition through MCP
    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        4,
        "add-cognition",
        serde_json::json!({
            "agent": "witness.process",
            "texture": "observation",
            "content": "Cross-layer test from MCP"
        }),
    )
    .await;

    assert!(!result["result"]["isError"].as_bool().unwrap_or(false));

    // Visible through CLI
    let cmd = app
        .command("cognition list --agent witness.process")
        .await?;
    let prompt = cmd.prompt();
    assert!(
        prompt.contains("Cross-layer test"),
        "cognition from MCP should be visible in CLI output: {prompt}"
    );

    // Visible through resource read
    let json = mcp_request(
        &http,
        &url,
        &session_id,
        5,
        "resources/read",
        Some(serde_json::json!({ "uri": "oneiros-mcp://agent/witness.process/cognitions" })),
    )
    .await;

    let text = json["result"]["contents"][0]["text"]
        .as_str()
        .unwrap_or_default();
    assert!(
        text.contains("Cross-layer test"),
        "cognition should be visible through resource read"
    );

    Ok(())
}

// ── Error paths ──────────────────────────────────────────────────

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

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        10,
        "tools/call",
        Some(serde_json::json!({
            "name": "nonexistent_tool",
            "arguments": {}
        })),
    )
    .await;

    // The result should indicate an error
    let is_error = json["result"]["isError"].as_bool().unwrap_or(false);
    assert!(
        is_error,
        "unknown tool should return isError=true, got: {json}"
    );

    // ── Bad parameters ──────────────────────────────────────────

    let json = mcp_request(
        &http,
        &url,
        &session_id,
        11,
        "tools/call",
        Some(serde_json::json!({
            "name": "get-agent",
            "arguments": { "wrong_field": "not a name" }
        })),
    )
    .await;

    // Should also indicate an error — deserialization failure or domain error
    let is_error = json["result"]["isError"].as_bool().unwrap_or(false);
    assert!(
        is_error,
        "bad params should return isError=true, got: {json}"
    );

    // ── Domain error through MCP ────────────────────────────────

    // Try to get a nonexistent agent — should be a domain-level error
    let json = mcp_request(
        &http,
        &url,
        &session_id,
        12,
        "tools/call",
        Some(serde_json::json!({
            "name": "get-agent",
            "arguments": { "name": "ghost.nobody" }
        })),
    )
    .await;

    let is_error = json["result"]["isError"].as_bool().unwrap_or(false);
    assert!(
        is_error,
        "nonexistent agent via MCP should return isError=true, got: {json}"
    );

    Ok(())
}
