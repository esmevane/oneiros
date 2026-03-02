mod common;
use common::*;

#[tokio::test]
async fn add_cognition_returns_created() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_texture(&state, &token, "observation").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "texture": "observation",
        "content": "The coupling between modules feels wrong."
    });

    let response = app
        .oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let cognition: Cognition = serde_json::from_slice(&bytes).unwrap();
    assert!(!cognition.id.is_empty());
    assert_eq!(cognition.texture, TextureName::new("observation"));
    assert_eq!(
        cognition.content.as_str(),
        "The coupling between modules feels wrong."
    );
}

#[tokio::test]
async fn add_cognition_requires_existing_agent() {
    let (_temp, state, token) = setup();
    seed_texture(&state, &token, "observation").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "nonexistent",
        "texture": "observation",
        "content": "This should fail."
    });

    let response = app
        .oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn add_cognition_requires_existing_texture() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "texture": "nonexistent",
        "content": "This should fail."
    });

    let response = app
        .oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_cognitions_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/cognitions", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Cognition> = serde_json::from_slice(&bytes).unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_cognitions_after_add() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_texture(&state, &token, "observation").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "texture": "observation",
        "content": "First thought."
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "texture": "observation",
        "content": "Second thought."
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/cognitions", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Cognition> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn list_cognitions_filtered_by_agent() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_agent(&state, &token, "editor", "expert").await;
    seed_texture(&state, &token, "observation").await;

    // Add cognition for architect
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "texture": "observation",
        "content": "Architect thought."
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Add cognition for editor
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "editor",
        "texture": "observation",
        "content": "Editor thought."
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Filter by architect
    let app = router(state);
    let response = app
        .oneshot(get_auth("/cognitions?agent=architect", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Cognition> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].content.as_str(), "Architect thought.");
}

#[tokio::test]
async fn list_cognitions_filtered_by_texture() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_texture(&state, &token, "observation").await;
    seed_texture(&state, &token, "insight").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "texture": "observation",
        "content": "An observation."
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "texture": "insight",
        "content": "An insight."
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Filter by insight
    let app = router(state);
    let response = app
        .oneshot(get_auth("/cognitions?texture=insight", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Cognition> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].content.as_str(), "An insight.");
}

#[tokio::test]
async fn get_cognition_not_found() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let fake_id = CognitionId::new();
    let response = app
        .oneshot(get_auth(&format!("/cognitions/{fake_id}"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_cognition_by_id() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_texture(&state, &token, "observation").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "texture": "observation",
        "content": "A notable observation."
    });
    let response = app
        .oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: Cognition = serde_json::from_slice(&bytes).unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/cognitions/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let fetched: Cognition = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.content.as_str(), "A notable observation.");
}

#[tokio::test]
async fn cognition_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/cognitions")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn cognition_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/cognitions", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
