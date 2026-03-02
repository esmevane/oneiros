mod common;
use common::*;

#[tokio::test]
async fn add_memory_returns_created() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_level(&state, &token, "core").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "The system uses event sourcing for all state changes."
    });

    let response = app
        .oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let memory: Memory = serde_json::from_slice(&bytes).unwrap();
    assert!(!memory.id.is_empty());
    assert_eq!(memory.level, LevelName::new("core"));
    assert_eq!(
        memory.content.as_str(),
        "The system uses event sourcing for all state changes."
    );
}

#[tokio::test]
async fn add_memory_requires_existing_agent() {
    let (_temp, state, token) = setup();
    seed_level(&state, &token, "core").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "nonexistent",
        "level": "core",
        "content": "This should fail."
    });

    let response = app
        .oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn add_memory_requires_existing_level() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "level": "nonexistent",
        "content": "This should fail."
    });

    let response = app
        .oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_memories_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/memories", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Memory> = serde_json::from_slice(&bytes).unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_memories_after_add() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_level(&state, &token, "core").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "First memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "Second memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/memories", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Memory> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn list_memories_filtered_by_agent() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_agent(&state, &token, "editor", "expert").await;
    seed_level(&state, &token, "core").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "Architect memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "editor",
        "level": "core",
        "content": "Editor memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth("/memories?agent=architect", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Memory> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].content.as_str(), "Architect memory.");
}

#[tokio::test]
async fn list_memories_filtered_by_level() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_level(&state, &token, "core").await;
    seed_level(&state, &token, "active").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "A core memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "active",
        "content": "An active memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth("/memories?level=active", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Memory> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].content.as_str(), "An active memory.");
}

#[tokio::test]
async fn get_memory_not_found() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let fake_id = MemoryId::new();
    let response = app
        .oneshot(get_auth(&format!("/memories/{fake_id}"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_memory_by_id() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_level(&state, &token, "core").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "A notable memory."
    });
    let response = app
        .oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: Memory = serde_json::from_slice(&bytes).unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/memories/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let fetched: Memory = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.content.as_str(), "A notable memory.");
}

#[tokio::test]
async fn memory_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/memories")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn memory_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/memories", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
