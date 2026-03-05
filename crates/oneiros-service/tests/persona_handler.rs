mod common;
use common::*;

#[tokio::test]
async fn set_persona_returns_ok() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let body = serde_json::json!({
        "name": "expert",
        "description": "A domain expert",
        "prompt": "You are a domain expert."
    });

    let response = app
        .oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let info: Persona = body_json(response).await;
    assert_eq!(info.name, PersonaName::new("expert"));
    assert_eq!(info.description.as_str(), "A domain expert");
    assert_eq!(info.prompt.as_str(), "You are a domain expert.");
}

#[tokio::test]
async fn set_persona_is_idempotent() {
    let (_temp, state, token) = setup();

    let body = serde_json::json!({
        "name": "expert",
        "description": "Version 1",
        "prompt": "Prompt v1"
    });

    // First set
    let app = router(state.clone());
    let response = app
        .oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Second set (update)
    let body2 = serde_json::json!({
        "name": "expert",
        "description": "Version 2",
        "prompt": "Prompt v2"
    });
    let app = router(state.clone());
    let response = app
        .oneshot(put_json_auth("/personas", &body2, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify updated
    let app = router(state);
    let response = app
        .oneshot(get_auth("/personas/expert", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let info: Persona = body_json(response).await;
    assert_eq!(info.description.as_str(), "Version 2");
    assert_eq!(info.prompt.as_str(), "Prompt v2");
}

#[tokio::test]
async fn list_personas_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/personas", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let list: Vec<Persona> = body_json(response).await;
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_personas_after_set() {
    let (_temp, state, token) = setup();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "alpha", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "beta", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/personas", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let list: Vec<Persona> = body_json(response).await;
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn get_persona_not_found() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/personas/nonexistent", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn remove_persona_then_gone() {
    let (_temp, state, token) = setup();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "ephemeral", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let response = app
        .oneshot(delete_auth("/personas/ephemeral", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let app = router(state);
    let response = app
        .oneshot(get_auth("/personas/ephemeral", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn persona_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    // No auth header at all
    let request = Request::builder()
        .method(Method::GET)
        .uri("/personas")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn persona_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/personas", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
