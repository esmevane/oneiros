use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use oneiros_db::Database;
use oneiros_link::*;
use oneiros_model::*;
use oneiros_service::*;
use std::sync::Arc;
use tempfile::TempDir;
use tower::util::ServiceExt;

fn seed_tenant_and_brain(db: &Database, brain_path: &std::path::Path) -> String {
    let tenant_id = TenantId::new();
    let actor_id = ActorId::new();

    let event = Events::Tenant(TenantEvents::TenantCreated(Identity::new(
        tenant_id,
        Tenant {
            name: TenantName::new("Test Tenant"),
        },
    )));
    db.log_event(&event, projections::system::ALL).unwrap();

    let event = Events::Actor(ActorEvents::ActorCreated(Identity::new(
        actor_id,
        Actor {
            tenant_id,
            name: ActorName::new("Test Actor"),
        },
    )));
    db.log_event(&event, projections::system::ALL).unwrap();

    Database::create_brain_db(brain_path).unwrap();

    let brain_id = BrainId::new();
    let event = Events::Brain(BrainEvents::BrainCreated(Identity::new(
        brain_id,
        HasPath::new(
            brain_path,
            Brain {
                tenant_id,
                name: BrainName::new("test-brain"),
                status: BrainStatus::Active,
            },
        ),
    )));

    db.log_event(&event, projections::system::ALL).unwrap();

    let token = Token::issue(TokenClaims {
        brain_id,
        tenant_id,
        actor_id,
    });

    let event = Events::Ticket(TicketEvents::TicketIssued(Identity::new(
        TicketId::new(),
        Ticket {
            token: token.clone(),
            created_by: actor_id,
        },
    )));
    db.log_event(&event, projections::system::ALL).unwrap();

    token.0
}

fn setup() -> (TempDir, Arc<ServiceState>, String) {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("service.db");
    let db = Database::create(db_path).unwrap();

    let brain_path = temp.path().join("brains").join("test-brain.db");
    std::fs::create_dir_all(brain_path.parent().unwrap()).unwrap();
    let token = seed_tenant_and_brain(&db, &brain_path);

    let state = Arc::new(ServiceState::new(db, temp.path().to_path_buf()));
    (temp, state, token)
}

fn get_auth(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

fn post_json_auth(uri: &str, body: &serde_json::Value, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

fn put_json_auth(uri: &str, body: &serde_json::Value, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::PUT)
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

/// Seed a persona and an agent in the brain, returning the agent name.
async fn seed_agent(state: &Arc<ServiceState>, token: &str, agent_name: &str, persona_name: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": persona_name,
        "description": "Test persona",
        "prompt": "You are a test persona."
    });
    app.oneshot(put_json_auth("/personas", &body, token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "name": agent_name,
        "persona": persona_name
    });
    app.oneshot(post_json_auth("/agents", &body, token))
        .await
        .unwrap();
}

/// Seed a sensation in the brain.
async fn seed_sensation(state: &Arc<ServiceState>, token: &str, sensation_name: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": sensation_name,
        "description": "Test sensation",
        "prompt": "Guidance for this sensation."
    });
    app.oneshot(put_json_auth("/sensations", &body, token))
        .await
        .unwrap();
}

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

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let experience: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();
    assert!(!experience.id.is_empty());
    assert_eq!(experience.sensation, SensationName::new("echoes"));
    assert_eq!(
        experience.description.as_str(),
        "Template changes and orientation gap are two approaches to the same problem."
    );
    assert!(experience.refs.is_empty());
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
async fn create_experience_with_refs() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_sensation(&state, &token, "caused").await;

    let fake_id = Id::new();
    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "caused",
        "description": "One insight led to another.",
        "refs": [
            { "id": fake_id.to_string(), "kind": "cognition", "role": "origin" }
        ]
    });

    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let experience: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(experience.refs.len(), 1);
    assert_eq!(
        experience.refs[0].kind().cloned(),
        Some(RecordKind::Cognition)
    );
    assert_eq!(
        experience.refs[0].role().map(|l| l.as_str()),
        Some("origin")
    );
}

#[tokio::test]
async fn list_experiences_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/experiences", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<ExperienceRecord> = serde_json::from_slice(&bytes).unwrap();
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

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<ExperienceRecord> = serde_json::from_slice(&bytes).unwrap();
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
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/experiences/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let fetched: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();
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
async fn add_ref_to_existing_experience() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_sensation(&state, &token, "caused").await;

    // Create experience without refs.
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "caused",
        "description": "A causal link."
    });
    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();
    assert!(created.refs.is_empty());

    // Add a ref.
    let ref_id = Id::new();
    let app = router(state.clone());
    let body = serde_json::json!({
        "id": ref_id.to_string(),
        "kind": "memory",
        "role": "origin"
    });
    let response = app
        .oneshot(post_json_auth(
            &format!("/experiences/{}/refs", created.id),
            &body,
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let updated: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(updated.refs.len(), 1);
    assert_eq!(updated.refs[0].kind().cloned(), Some(RecordKind::Memory));
    assert_eq!(updated.refs[0].role().map(|l| l.as_str()), Some("origin"));
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
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();

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

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let updated: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        updated.description.as_str(),
        "Updated description with deeper understanding."
    );
}

#[tokio::test]
async fn create_experience_with_linked_ref() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_sensation(&state, &token, "echoes").await;

    let link = Link::new(&("cognition", "some-thought")).unwrap();
    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "echoes",
        "description": "A resonance traced by link.",
        "refs": [
            { "link": link.to_string(), "role": "origin" }
        ]
    });

    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let experience: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(experience.refs.len(), 1);
    assert_eq!(experience.refs[0].link(), Some(&link));
    assert!(experience.refs[0].id().is_none());
    assert!(experience.refs[0].kind().is_none());
    assert_eq!(
        experience.refs[0].role().map(|l| l.as_str()),
        Some("origin")
    );
}

#[tokio::test]
async fn add_linked_ref_to_existing_experience() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_sensation(&state, &token, "caused").await;

    // Create experience without refs.
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "caused",
        "description": "A causal link traced by content address."
    });
    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();
    assert!(created.refs.is_empty());

    // Add a linked ref.
    let link = Link::new(&("memory", "a-consolidated-insight")).unwrap();
    let app = router(state.clone());
    let body = serde_json::json!({
        "link": link.to_string(),
        "role": "origin"
    });
    let response = app
        .oneshot(post_json_auth(
            &format!("/experiences/{}/refs", created.id),
            &body,
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let updated: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(updated.refs.len(), 1);
    assert_eq!(updated.refs[0].link(), Some(&link));
    assert_eq!(updated.refs[0].role().map(|l| l.as_str()), Some("origin"));
}

#[tokio::test]
async fn create_experience_with_mixed_refs() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_sensation(&state, &token, "tensions").await;

    let fake_id = Id::new();
    let link = Link::new(&("cognition", "another-thought")).unwrap();
    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "sensation": "tensions",
        "description": "Two poles pulling against each other.",
        "refs": [
            { "id": fake_id.to_string(), "kind": "cognition", "role": "pole-a" },
            { "link": link.to_string(), "role": "pole-b" }
        ]
    });

    let response = app
        .oneshot(post_json_auth("/experiences", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let experience: ExperienceRecord = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(experience.refs.len(), 2);
    // First ref is identified
    assert!(experience.refs[0].id().is_some());
    assert_eq!(
        experience.refs[0].role().map(|l| l.as_str()),
        Some("pole-a")
    );
    // Second ref is linked
    assert_eq!(experience.refs[1].link(), Some(&link));
    assert_eq!(
        experience.refs[1].role().map(|l| l.as_str()),
        Some("pole-b")
    );
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
