mod common;
use common::*;

async fn seed_search_agent(state: &Arc<ServiceState>, token: &str) {
    seed_agent(state, token, "searcher", "tester").await;
    seed_texture(state, token, "tester").await;
}

#[tokio::test]
async fn search_finds_cognition_content() {
    let (_temp, state, token) = setup();
    seed_search_agent(&state, &token).await;

    // Add a cognition
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "searcher",
        "texture": "tester",
        "content": "The quick brown fox jumps over the lazy dog"
    });
    let response = app
        .oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Search for it
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?query=fox", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let results: SearchResults = body_json(response).await;

    assert_eq!(results.query, "fox");
    assert_eq!(results.results.len(), 1);
    assert_eq!(results.results[0].kind.as_str(), "cognition-content");
    assert!(results.results[0].content.as_str().contains("fox"));
}

#[tokio::test]
async fn search_returns_empty_for_no_match() {
    let (_temp, state, token) = setup();
    seed_search_agent(&state, &token).await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "searcher",
        "texture": "tester",
        "content": "Hello world"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?query=nonexistent", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let results: SearchResults = body_json(response).await;

    assert!(results.results.is_empty());
}

#[tokio::test]
async fn search_finds_agent_description() {
    let (_temp, state, token) = setup();
    seed_search_agent(&state, &token).await;

    // Update agent with a searchable description
    let app = router(state.clone());
    let body = serde_json::json!({
        "persona": "tester",
        "description": "A specialized quantum computing researcher",
        "prompt": "Think about qubits."
    });
    let response = app
        .oneshot(put_json_auth("/agents/searcher", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Search for the description
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?query=quantum", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let results: SearchResults = body_json(response).await;

    assert!(!results.results.is_empty());
    let kinds: Vec<&str> = results.results.iter().map(|r| r.kind.as_str()).collect();
    assert!(kinds.contains(&"agent-description"));
}

#[tokio::test]
async fn search_finds_persona_content() {
    let (_temp, state, token) = setup();

    // Create persona with searchable description
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "philosopher",
        "description": "Contemplates epistemological foundations",
        "prompt": "Think deeply about knowledge."
    });
    let response = app
        .oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Search for the description
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?query=epistemological", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let results: SearchResults = body_json(response).await;

    assert_eq!(results.results.len(), 1);
    assert_eq!(results.results[0].kind.as_str(), "persona-description");
}

#[tokio::test]
async fn search_across_multiple_entity_types() {
    let (_temp, state, token) = setup();
    seed_search_agent(&state, &token).await;

    // Add cognition with "architecture"
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "searcher",
        "texture": "tester",
        "content": "The architecture of this system is elegant"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Update agent description with "architecture"
    let app = router(state.clone());
    let body = serde_json::json!({
        "persona": "tester",
        "description": "An architecture expert agent",
        "prompt": "Think about systems."
    });
    app.oneshot(put_json_auth("/agents/searcher", &body, &token))
        .await
        .unwrap();

    // Search — should find both
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?query=architecture", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let results: SearchResults = body_json(response).await;

    assert!(results.results.len() >= 2);
    let kinds: Vec<&str> = results.results.iter().map(|r| r.kind.as_str()).collect();
    assert!(kinds.contains(&"cognition-content"));
    assert!(kinds.contains(&"agent-description"));
}

#[tokio::test]
async fn search_scoped_to_agent_returns_matching() {
    let (_temp, state, token) = setup();
    seed_search_agent(&state, &token).await;
    seed_agent(&state, &token, "other-agent", "tester").await;

    // Add cognition for searcher
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "searcher",
        "texture": "tester",
        "content": "Quantum mechanics is fundamental"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Add cognition for other-agent with same keyword
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "other-agent",
        "texture": "tester",
        "content": "Quantum computing advances rapidly"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Search WITHOUT agent filter — should find both
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?query=quantum", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let all_results: SearchResults = body_json(response).await;
    assert_eq!(all_results.results.len(), 2);

    // Search WITH agent filter — should find only searcher's
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?query=quantum&agent=searcher", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let filtered_results: SearchResults = body_json(response).await;
    assert_eq!(filtered_results.results.len(), 1);
    assert!(
        filtered_results.results[0]
            .content
            .as_str()
            .contains("mechanics")
    );
}

#[tokio::test]
async fn search_without_agent_returns_all() {
    let (_temp, state, token) = setup();
    seed_search_agent(&state, &token).await;
    seed_agent(&state, &token, "other-agent", "tester").await;

    // Add cognitions for both agents
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "searcher",
        "texture": "tester",
        "content": "Exploring neural pathways"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "other-agent",
        "texture": "tester",
        "content": "Neural network architectures"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Search without agent filter
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?query=neural", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let results: SearchResults = body_json(response).await;

    // Both agents' cognitions should appear
    assert_eq!(results.results.len(), 2);
}
