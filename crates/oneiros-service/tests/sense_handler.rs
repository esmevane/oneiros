mod common;
use common::*;

#[tokio::test]
async fn sense_returns_agent() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "governor.process", "process").await;

    let app = router(state);
    let response = app
        .oneshot(post_auth("/sense/governor.process", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let agent: Agent = body_json(response).await;
    assert_eq!(agent.name, AgentName::new("governor.process"));
}

#[tokio::test]
async fn sense_returns_not_found_for_missing_agent() {
    let (_temp, state, token) = setup();

    let app = router(state);
    let response = app
        .oneshot(post_auth("/sense/nonexistent", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn sense_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::POST)
        .uri("/sense/governor.process")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
