//! MCP workflow — tool discovery and execution via the Model Context Protocol.
//!
//! MCP clients connect over streamable HTTP, initialize a session, discover
//! available tools, and call them. Data created through MCP is visible
//! through all other layers (HTTP REST, CLI).

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
    if let Some(ref sid) = session_id {
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
    if let Some(ref sid) = session_id {
        notif = notif.header("mcp-session-id", sid);
    }
    notif.send().await.unwrap();

    session_id
}

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

    // Discover tools
    let mut tools_req = mcp_post(&http, &url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    }));
    if let Some(ref sid) = session_id {
        tools_req = tools_req.header("mcp-session-id", sid);
    }
    let tools_resp = tools_req.send().await?;
    assert!(tools_resp.status().is_success());

    let body = tools_resp.text().await?;
    let json = extract_sse_json(&body);
    let tools = json["result"]["tools"]
        .as_array()
        .expect("tools should be an array");

    // All 68 tools should be listed
    assert!(
        tools.len() >= 60,
        "expected at least 60 tools, got {}",
        tools.len()
    );

    // Spot-check across domains
    let tool_names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
    assert!(tool_names.contains(&"create-agent"));
    assert!(tool_names.contains(&"dream-agent"));
    assert!(tool_names.contains(&"search-query"));
    assert!(tool_names.contains(&"add-cognition"));
    assert!(tool_names.contains(&"set-level"));
    assert!(tool_names.contains(&"list-storage"));

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
    for name in &tool_names {
        assert!(seen.insert(name), "duplicate tool name: {name}");
    }

    // Call a tool — create a persona through MCP
    let mut call_req = mcp_post(&http, &url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "set-persona",
            "arguments": {
                "name": "mcp-persona",
                "description": "Created via MCP",
                "prompt": "You were born from a tool call"
            }
        }
    }));
    if let Some(ref sid) = session_id {
        call_req = call_req.header("mcp-session-id", sid);
    }
    let call_resp = call_req.send().await?;
    assert!(call_resp.status().is_success());

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
    if let Some(ref sid) = session_id {
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

    // ── Bad parameters ──────────────────────────────────────────

    let mut req = mcp_post(&http, &url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 11,
        "method": "tools/call",
        "params": {
            "name": "get-agent",
            "arguments": { "wrong_field": "not a name" }
        }
    }));
    if let Some(ref sid) = session_id {
        req = req.header("mcp-session-id", sid);
    }
    let resp = req.send().await?;
    assert!(resp.status().is_success());
    let body = resp.text().await?;
    let json = extract_sse_json(&body);

    // Should also indicate an error — deserialization failure or domain error
    let is_error = json["result"]["isError"].as_bool().unwrap_or(false);
    assert!(
        is_error,
        "bad params should return isError=true, got: {json}"
    );

    // ── Domain error through MCP ────────────────────────────────

    // Try to get a nonexistent agent — should be a domain-level error
    let mut req = mcp_post(&http, &url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 12,
        "method": "tools/call",
        "params": {
            "name": "get-agent",
            "arguments": { "name": "ghost.nobody" }
        }
    }));
    if let Some(ref sid) = session_id {
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
