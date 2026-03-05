mod common;
use common::*;

#[tokio::test]
async fn create_experience_returns_created() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_sensation(&state, &token, "echoes").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "echoes",
        "description": "Template changes and orientation gap are two approaches to the same problem."
    });

    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let experience: Experience = body_json(response).await;
    assert!(!experience.id.is_empty());
    assert_eq!(experience.sensation, SensationName::new("echoes"));
    assert_eq!(
        experience.description.as_str(),
        "Template changes and orientation gap are two approaches to the same problem."
    );
}

#[tokio::test]
async fn create_experience_requires_existing_agent() {
    let (_temp, state, token) = setup();
    seed_sensation(&state, &token, "echoes").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "nonexistent",
        "sensation": "echoes",
        "description": "This should fail."
    });

    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn create_experience_requires_existing_sensation() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "nonexistent",
        "description": "This should fail."
    });

    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_experiences_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/experiences", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let list: Vec<Experience> = body_json(response).await;
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_experiences_after_create() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_sensation(&state, &token, "echoes").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "echoes",
        "description": "First experience."
    });
    app.oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "echoes",
        "description": "Second experience."
    });
    app.oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/experiences", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let list: Vec<Experience> = body_json(response).await;
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn get_experience_by_id() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_sensation(&state, &token, "echoes").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "echoes",
        "description": "A notable resonance."
    });
    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    let created: Experience = body_json(response).await;

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/experiences/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let fetched: Experience = body_json(response).await;
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.description.as_str(), "A notable resonance.");
}

#[tokio::test]
async fn get_experience_not_found() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let fake_id = ExperienceId::new();
    let response = app
        .oneshot(get_auth(&format!("/experiences/{fake_id}"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn update_experience_description() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_sensation(&state, &token, "tensions").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "tensions",
        "description": "Original description."
    });
    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    let created: Experience = body_json(response).await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "description": "Updated description with deeper understanding."
    });
    let response = app
        .oneshot(put_json_auth(
            &format!("/experiences/{}/description", created.id),
            &body,
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let updated: Experience = body_json(response).await;
    assert_eq!(
        updated.description.as_str(),
        "Updated description with deeper understanding."
    );
}

#[tokio::test]
async fn update_experience_sensation() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_sensation(&state, &token, "tensions").await;
    seed_sensation(&state, &token, "echoes").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "tensions",
        "description": "Original experience."
    });
    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    let created: Experience = body_json(response).await;
    assert_eq!(created.sensation, SensationName::new("tensions"));

    let app = router(state.clone());
    let body = serde_json::json!({
        "sensation": "echoes"
    });
    let response = app
        .oneshot(put_json_auth(
            &format!("/experiences/{}/sensation", created.id),
            &body,
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let updated: Experience = body_json(response).await;
    assert_eq!(updated.sensation, SensationName::new("echoes"));
    assert_eq!(updated.description.as_str(), "Original experience.");
}

#[tokio::test]
async fn experience_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/experiences")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn experience_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/experiences", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
