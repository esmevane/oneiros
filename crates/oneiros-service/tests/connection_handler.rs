mod common;
use common::*;

#[tokio::test]
async fn create_connection_returns_created() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "origin").await;

    let ref_a = Ref::agent(AgentId::new());
    let ref_b = Ref::cognition(CognitionId::new());

    let body = serde_json::json!({
        "nature": "origin",
        "from_ref": serde_json::to_value(&ref_a).unwrap(),
        "to_ref": serde_json::to_value(&ref_b).unwrap(),
    });

    let app = router(state);
    let response = app
        .oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let connection: Connection = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(connection.nature, NatureName::new("origin"));
    assert_eq!(connection.from_ref, ref_a);
    assert_eq!(connection.to_ref, ref_b);
}

#[tokio::test]
async fn create_connection_with_unknown_nature_returns_not_found() {
    let (_temp, state, token) = setup();

    let ref_a = Ref::agent(AgentId::new());
    let ref_b = Ref::cognition(CognitionId::new());

    let body = serde_json::json!({
        "nature": "nonexistent",
        "from_ref": serde_json::to_value(&ref_a).unwrap(),
        "to_ref": serde_json::to_value(&ref_b).unwrap(),
    });

    let app = router(state);
    let response = app
        .oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_connections_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/connections", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Connection> = serde_json::from_slice(&bytes).unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_connections_after_create() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "origin").await;

    let ref_a = Ref::agent(AgentId::new());
    let ref_b = Ref::cognition(CognitionId::new());

    let body = serde_json::json!({
        "nature": "origin",
        "from_ref": serde_json::to_value(&ref_a).unwrap(),
        "to_ref": serde_json::to_value(&ref_b).unwrap(),
    });

    let app = router(state.clone());
    app.oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/connections", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Connection> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 1);
}

#[tokio::test]
async fn get_connection_not_found() {
    let (_temp, state, token) = setup();
    let fake_id = ConnectionId::new();
    let app = router(state);

    let response = app
        .oneshot(get_auth(&format!("/connections/{fake_id}"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn show_connection_by_id() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "context").await;

    let ref_a = Ref::agent(AgentId::new());
    let ref_b = Ref::memory(MemoryId::new());

    let body = serde_json::json!({
        "nature": "context",
        "from_ref": serde_json::to_value(&ref_a).unwrap(),
        "to_ref": serde_json::to_value(&ref_b).unwrap(),
    });

    let app = router(state.clone());
    let response = app
        .oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: Connection = serde_json::from_slice(&bytes).unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/connections/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let fetched: Connection = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.nature, NatureName::new("context"));
}

#[tokio::test]
async fn remove_connection_then_gone() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "origin").await;

    let ref_a = Ref::agent(AgentId::new());
    let ref_b = Ref::cognition(CognitionId::new());

    let body = serde_json::json!({
        "nature": "origin",
        "from_ref": serde_json::to_value(&ref_a).unwrap(),
        "to_ref": serde_json::to_value(&ref_b).unwrap(),
    });

    let app = router(state.clone());
    let response = app
        .oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: Connection = serde_json::from_slice(&bytes).unwrap();

    let app = router(state.clone());
    let response = app
        .oneshot(delete_auth(&format!("/connections/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/connections/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_connections_filters_by_nature() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "origin").await;
    seed_nature(&state, &token, "context").await;

    let ref_a = Ref::agent(AgentId::new());
    let ref_b = Ref::cognition(CognitionId::new());

    // Create one origin and one context connection.
    let body = serde_json::json!({
        "nature": "origin",
        "from_ref": serde_json::to_value(&ref_a).unwrap(),
        "to_ref": serde_json::to_value(&ref_b).unwrap(),
    });
    let app = router(state.clone());
    app.oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();

    let body = serde_json::json!({
        "nature": "context",
        "from_ref": serde_json::to_value(&ref_a).unwrap(),
        "to_ref": serde_json::to_value(&ref_b).unwrap(),
    });
    let app = router(state.clone());
    app.oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();

    // Filter by nature=origin should return 1.
    let app = router(state);
    let response = app
        .oneshot(get_auth("/connections?nature=origin", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Connection> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].nature, NatureName::new("origin"));
}

#[tokio::test]
async fn connection_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/connections")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn connection_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/connections", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
