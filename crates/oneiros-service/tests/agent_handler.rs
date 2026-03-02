mod common;
use common::*;

#[tokio::test]
async fn create_agent_returns_created() {
    let (_temp, state, token) = setup();
    ensure_persona(
        &state,
        &token,
        "expert",
        "A domain expert",
        "You are a domain expert.",
    )
    .await;

    let app = router(state);
    let body = serde_json::json!({
        "name": "architect",
        "persona": "expert",
        "description": "The system architect",
        "prompt": "You design systems."
    });

    let response = app
        .oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let agent: Agent = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(agent.name, AgentName::new("architect"));
    assert_eq!(agent.persona, PersonaName::new("expert"));
    assert_eq!(agent.description.as_str(), "The system architect");
    assert_eq!(agent.prompt.as_str(), "You design systems.");
    assert!(!agent.id.is_empty());
}

#[tokio::test]
async fn create_agent_requires_existing_persona() {
    let (_temp, state, token) = setup();
    // Do NOT seed any persona.

    let app = router(state);
    let body = serde_json::json!({
        "name": "orphan",
        "persona": "nonexistent"
    });

    let response = app
        .oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn create_agent_conflict_on_duplicate_name() {
    let (_temp, state, token) = setup();
    ensure_persona(
        &state,
        &token,
        "expert",
        "A domain expert",
        "You are a domain expert.",
    )
    .await;

    let body = serde_json::json!({
        "name": "architect",
        "persona": "expert"
    });

    let app = router(state.clone());
    let response = app
        .oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let app = router(state);
    let response = app
        .oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn list_agents_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/agents", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Agent> = serde_json::from_slice(&bytes).unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_agents_after_create() {
    let (_temp, state, token) = setup();
    ensure_persona(
        &state,
        &token,
        "expert",
        "A domain expert",
        "You are a domain expert.",
    )
    .await;

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "alpha", "persona": "expert" });
    app.oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "beta", "persona": "expert" });
    app.oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/agents", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Agent> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn get_agent_not_found() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/agents/nonexistent", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_agent_by_name() {
    let (_temp, state, token) = setup();
    ensure_persona(
        &state,
        &token,
        "expert",
        "A domain expert",
        "You are a domain expert.",
    )
    .await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "architect",
        "persona": "expert",
        "description": "The architect",
        "prompt": "Design things."
    });
    app.oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth("/agents/architect", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let agent: Agent = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(agent.name, AgentName::new("architect"));
    assert_eq!(agent.persona, PersonaName::new("expert"));
}

#[tokio::test]
async fn update_agent() {
    let (_temp, state, token) = setup();
    ensure_persona(
        &state,
        &token,
        "expert",
        "A domain expert",
        "You are a domain expert.",
    )
    .await;

    // Create
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "architect",
        "persona": "expert",
        "description": "Version 1",
        "prompt": "Prompt v1"
    });
    app.oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();

    // Update
    let app = router(state.clone());
    let update_body = serde_json::json!({
        "persona": "expert",
        "description": "Version 2",
        "prompt": "Prompt v2"
    });
    let response = app
        .oneshot(put_json_auth("/agents/architect", &update_body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify
    let app = router(state);
    let response = app
        .oneshot(get_auth("/agents/architect", &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let agent: Agent = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(agent.description.as_str(), "Version 2");
    assert_eq!(agent.prompt.as_str(), "Prompt v2");
}

#[tokio::test]
async fn update_nonexistent_agent() {
    let (_temp, state, token) = setup();
    ensure_persona(
        &state,
        &token,
        "expert",
        "A domain expert",
        "You are a domain expert.",
    )
    .await;

    let app = router(state);
    let update_body = serde_json::json!({
        "persona": "expert",
        "description": "Nope",
        "prompt": "Nope"
    });
    let response = app
        .oneshot(put_json_auth("/agents/ghost", &update_body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn remove_agent_then_gone() {
    let (_temp, state, token) = setup();
    ensure_persona(
        &state,
        &token,
        "expert",
        "A domain expert",
        "You are a domain expert.",
    )
    .await;

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "ephemeral", "persona": "expert" });
    app.oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let response = app
        .oneshot(delete_auth("/agents/ephemeral", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let app = router(state);
    let response = app
        .oneshot(get_auth("/agents/ephemeral", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn agent_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/agents")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn agent_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/agents", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
