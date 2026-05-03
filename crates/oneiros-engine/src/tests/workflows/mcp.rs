//! MCP workflow — tools, resources, and prompts via the Model Context Protocol.
//!
//! Exercises the full MCP surface: 8 tools, 19 resources, 1 prompt.
//! All responses are authored markdown with navigational hints.

use crate::tests::harness::TestApp;
use crate::*;

fn mcp_post(http: &reqwest::Client, url: &str) -> reqwest::RequestBuilder {
    http.post(url)
        .header("Accept", "application/json, text/event-stream")
        .header("Content-Type", "application/json")
}

fn extract_sse_json(body: &str) -> serde_json::Value {
    let json_str = body
        .lines()
        .filter(|line| line.starts_with("data:"))
        .map(|line| line.strip_prefix("data:").unwrap().trim())
        .find(|s| !s.is_empty())
        .expect("response should contain a non-empty data: line");

    serde_json::from_str(json_str).expect("data: line should be valid JSON")
}

async fn mcp_initialize_raw(
    http: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
) -> serde_json::Value {
    let mut req = mcp_post(http, url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    }));
    if let Some((name, value)) = auth_header {
        req = req.header(name, value);
    }
    let resp = req.send().await.unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().await.unwrap();
    extract_sse_json(&body)
}

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

    assert!(init_resp.status().is_success());

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

async fn initialize_should_fail(
    http: &reqwest::Client,
    url: &str,
    auth_header: Option<(&str, &str)>,
    expected_substr: &str,
) {
    let json = mcp_initialize_raw(http, url, auth_header).await;
    let error = json
        .get("error")
        .unwrap_or_else(|| panic!("expected JSON-RPC error, got: {json}"));
    let message = error["message"]
        .as_str()
        .unwrap_or_else(|| panic!("error should have message, got: {error}"));
    assert!(
        message.contains(expected_substr),
        "error message should contain '{expected_substr}', got: '{message}'"
    );
}

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

async fn mcp_rpc(
    http: &reqwest::Client,
    url: &str,
    session_id: &Option<String>,
    id: u64,
    method: &str,
    params: serde_json::Value,
) -> serde_json::Value {
    let mut req = mcp_post(http, url).json(&serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params
    }));
    if let Some(sid) = session_id {
        req = req.header("mcp-session-id", sid);
    }
    let resp = req.send().await.unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().await.unwrap();
    extract_sse_json(&body)
}

fn tool_text(result: &serde_json::Value) -> String {
    result["result"]["content"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string()
}

fn resource_text(result: &serde_json::Value) -> String {
    result["result"]["contents"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string()
}

// ── Tool surface ────────────────────────────────────────────────

#[tokio::test]
async fn tools_discovery() -> Result<(), Box<dyn core::error::Error>> {
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
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    let json = mcp_rpc(
        &http,
        &url,
        &session_id,
        2,
        "tools/list",
        serde_json::json!({}),
    )
    .await;

    let tools = json["result"]["tools"]
        .as_array()
        .expect("tools should be an array");

    assert_eq!(
        tools.len(),
        8,
        "expected exactly 8 tools, got {}",
        tools.len()
    );

    let tool_names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
    assert!(tool_names.contains(&"create-agent"));
    assert!(tool_names.contains(&"update-agent"));
    assert!(tool_names.contains(&"remove-agent"));
    assert!(tool_names.contains(&"add-cognition"));
    assert!(tool_names.contains(&"add-memory"));
    assert!(tool_names.contains(&"create-experience"));
    assert!(tool_names.contains(&"create-connection"));
    assert!(tool_names.contains(&"search-query"));

    for tool in tools {
        assert!(tool["name"].is_string());
        assert!(tool["inputSchema"].is_object());
    }

    let mut seen = std::collections::HashSet::new();
    for name in &tool_names {
        assert!(seen.insert(name), "duplicate tool name: {name}");
    }

    Ok(())
}

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

    let token = app.token().expect("project should have a token");
    let http = reqwest::Client::new();
    let url = app.mcp_url();
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "search-query",
        serde_json::json!({ "query": "test" }),
    )
    .await;

    let text = tool_text(&result);
    assert!(
        text.contains("# Search:"),
        "search result should be markdown, got: {text}"
    );

    Ok(())
}

#[tokio::test]
async fn tool_auth_session() -> Result<(), Box<dyn core::error::Error>> {
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
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        3,
        "search-query",
        serde_json::json!({ "query": "test" }),
    )
    .await;

    let is_error = result["result"]["isError"].as_bool().unwrap_or(false);
    assert!(
        !is_error,
        "authenticated search should succeed, got: {result}"
    );

    Ok(())
}

// ── Resource surface ────────────────────────────────────────────

#[tokio::test]
async fn resources_discovery() -> Result<(), Box<dyn core::error::Error>> {
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
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    let json = mcp_rpc(
        &http,
        &url,
        &session_id,
        2,
        "resources/list",
        serde_json::json!({}),
    )
    .await;

    let resources = json["result"]["resources"]
        .as_array()
        .expect("resources should be an array");

    assert_eq!(
        resources.len(),
        9,
        "expected 9 concrete resources, got {}",
        resources.len()
    );

    let uris: Vec<&str> = resources.iter().filter_map(|r| r["uri"].as_str()).collect();
    assert!(uris.contains(&"oneiros-mcp://agents"));
    assert!(uris.contains(&"oneiros-mcp://levels"));
    assert!(uris.contains(&"oneiros-mcp://textures"));
    assert!(uris.contains(&"oneiros-mcp://sensations"));
    assert!(uris.contains(&"oneiros-mcp://natures"));
    assert!(uris.contains(&"oneiros-mcp://personas"));
    assert!(uris.contains(&"oneiros-mcp://urges"));
    assert!(uris.contains(&"oneiros-mcp://status"));
    assert!(uris.contains(&"oneiros-mcp://pressure"));

    let json = mcp_rpc(
        &http,
        &url,
        &session_id,
        3,
        "resources/templates/list",
        serde_json::json!({}),
    )
    .await;

    let templates = json["result"]["resourceTemplates"]
        .as_array()
        .expect("resource templates should be an array");

    assert_eq!(
        templates.len(),
        10,
        "expected 10 resource templates, got {}",
        templates.len()
    );

    Ok(())
}

#[tokio::test]
async fn resource_reads_return_markdown_with_hints() -> Result<(), Box<dyn core::error::Error>> {
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
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    let json = mcp_rpc(
        &http,
        &url,
        &session_id,
        10,
        "resources/read",
        serde_json::json!({ "uri": "oneiros-mcp://agents" }),
    )
    .await;

    let text = resource_text(&json);
    assert!(
        text.contains("# Agents"),
        "agents resource should be markdown, got: {text}"
    );
    assert!(
        text.contains("Hints"),
        "agents resource should have hints, got: {text}"
    );

    let mime = json["result"]["contents"][0]["mimeType"]
        .as_str()
        .unwrap_or("");
    assert_eq!(mime, "text/markdown");

    let json = mcp_rpc(
        &http,
        &url,
        &session_id,
        11,
        "resources/read",
        serde_json::json!({ "uri": "oneiros-mcp://levels" }),
    )
    .await;

    let text = resource_text(&json);
    assert!(
        text.contains("# Levels"),
        "levels resource should be markdown"
    );
    assert!(text.contains("Hints"), "levels resource should have hints");

    let json = mcp_rpc(
        &http,
        &url,
        &session_id,
        12,
        "resources/read",
        serde_json::json!({ "uri": "oneiros-mcp://status" }),
    )
    .await;

    let text = resource_text(&json);
    assert!(
        text.contains("# Agent Status"),
        "status resource should be markdown"
    );
    assert!(text.contains("Hints"), "status resource should have hints");

    Ok(())
}

// ── Prompt surface ──────────────────────────────────────────────

#[tokio::test]
async fn prompts_discovery() -> Result<(), Box<dyn core::error::Error>> {
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
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    let json = mcp_rpc(
        &http,
        &url,
        &session_id,
        2,
        "prompts/list",
        serde_json::json!({}),
    )
    .await;

    let prompts = json["result"]["prompts"]
        .as_array()
        .expect("prompts should be an array");

    assert_eq!(prompts.len(), 1, "expected 1 prompt, got {}", prompts.len());
    assert_eq!(prompts[0]["name"].as_str(), Some("dream"));
    assert!(prompts[0]["arguments"].is_array());

    Ok(())
}

#[tokio::test]
async fn dream_prompt_returns_agent_context() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command("seed agents").await?;

    let token = app.token().expect("project should have a token");
    let http = reqwest::Client::new();
    let url = app.mcp_url();
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    #[expect(
        unused_assignments,
        reason = "This is tantamount to putting `None` somewhere"
    )]
    let mut json = serde_json::Value::Null;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    let mut request_id = 5;
    loop {
        json = mcp_rpc(
            &http,
            &url,
            &session_id,
            request_id,
            "prompts/get",
            serde_json::json!({
                "name": "dream",
                "arguments": { "agent": "governor.process" }
            }),
        )
        .await;

        if json["result"]["messages"].is_array() {
            break;
        }

        if std::time::Instant::now() >= deadline {
            panic!("dream should return messages within 2s, got: {json}");
        }

        request_id += 1;
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    let messages = json["result"]["messages"]
        .as_array()
        .unwrap_or_else(|| panic!("dream should return messages, got: {json}"));

    assert!(
        !messages.is_empty(),
        "dream should have at least one message"
    );

    let text = messages[0]["content"]["text"].as_str().unwrap_or("");
    assert!(
        text.contains("governor.process"),
        "dream should contain agent name, got: {}",
        &text[..text.len().min(200)]
    );

    Ok(())
}

// ── Error paths ─────────────────────────────────────────────────

#[tokio::test]
async fn error_responses_include_hints() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let token = app.token().expect("project should have a token");
    let http = reqwest::Client::new();
    let url = app.mcp_url();
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        10,
        "nonexistent_tool",
        serde_json::json!({}),
    )
    .await;

    let is_error = result["result"]["isError"].as_bool().unwrap_or(false);
    assert!(is_error, "unknown tool should be an error");

    let text = tool_text(&result);
    assert!(
        text.contains("Hints"),
        "error response should include hints, got: {text}"
    );

    let result = mcp_call_tool(
        &http,
        &url,
        &session_id,
        11,
        "add-cognition",
        serde_json::json!({ "wrong_field": "not valid" }),
    )
    .await;

    let is_error = result["result"]["isError"].as_bool().unwrap_or(false);
    assert!(is_error, "bad params should be an error");

    let text = tool_text(&result);
    assert!(
        text.contains("Hints"),
        "malformed error should include hints, got: {text}"
    );

    Ok(())
}

#[tokio::test]
async fn invalid_resource_uri_returns_error() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let token = app.token().expect("project should have a token");
    let http = reqwest::Client::new();
    let url = app.mcp_url();
    let session_id = mcp_initialize_with_token(&http, &url, &token).await;

    let json = mcp_rpc(
        &http,
        &url,
        &session_id,
        10,
        "resources/read",
        serde_json::json!({ "uri": "oneiros-mcp://nonexistent" }),
    )
    .await;

    assert!(
        json["error"].is_object(),
        "invalid resource URI should return an error, got: {json}"
    );

    Ok(())
}

// ── Auth failure paths ──────────────────────────────────────────

#[tokio::test]
async fn initialize_without_token_fails() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let http = reqwest::Client::new();
    let url = app.mcp_url();

    initialize_should_fail(&http, &url, None, "oneiros mcp init").await;

    Ok(())
}

#[tokio::test]
async fn initialize_with_bad_token_fails() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let http = reqwest::Client::new();
    let url = app.mcp_url();

    initialize_should_fail(
        &http,
        &url,
        Some(("authorization", "Bearer garbage-token")),
        "Invalid or unrecognized",
    )
    .await;

    Ok(())
}

#[tokio::test]
async fn initialize_with_non_bearer_scheme_fails() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let http = reqwest::Client::new();
    let url = app.mcp_url();

    initialize_should_fail(
        &http,
        &url,
        Some(("authorization", "Basic dXNlcjpwYXNz")),
        "Invalid authorization scheme",
    )
    .await;

    Ok(())
}
