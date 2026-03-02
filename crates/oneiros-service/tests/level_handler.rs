mod common;
use common::*;

#[tokio::test]
async fn set_level_returns_ok() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let body = serde_json::json!({
        "name": "core",
        "description": "Identity-defining memories, always included",
        "prompt": "Only assign core status to foundational identity memories."
    });

    let response = app
        .oneshot(put_json_auth("/levels", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let info: Level = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(info.name, LevelName::new("core"));
    assert_eq!(
        info.description.as_str(),
        "Identity-defining memories, always included"
    );
}

#[tokio::test]
async fn set_level_is_idempotent() {
    let (_temp, state, token) = setup();

    let body = serde_json::json!({
        "name": "active",
        "description": "Version 1",
        "prompt": "Prompt v1"
    });

    let app = router(state.clone());
    let response = app
        .oneshot(put_json_auth("/levels", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body2 = serde_json::json!({
        "name": "active",
        "description": "Version 2",
        "prompt": "Prompt v2"
    });
    let app = router(state.clone());
    let response = app
        .oneshot(put_json_auth("/levels", &body2, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let app = router(state);
    let response = app
        .oneshot(get_auth("/levels/active", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let info: Level = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(info.description.as_str(), "Version 2");
    assert_eq!(info.prompt.as_str(), "Prompt v2");
}

#[tokio::test]
async fn list_levels_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/levels", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Level> = serde_json::from_slice(&bytes).unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_levels_after_set() {
    let (_temp, state, token) = setup();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "core", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/levels", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "archived", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/levels", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/levels", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Level> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn get_level_not_found() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/levels/nonexistent", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn remove_level_then_gone() {
    let (_temp, state, token) = setup();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "ephemeral", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/levels", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let response = app
        .oneshot(delete_auth("/levels/ephemeral", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let app = router(state);
    let response = app
        .oneshot(get_auth("/levels/ephemeral", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn level_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/levels")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn level_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/levels", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
