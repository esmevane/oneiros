use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use oneiros_db::Database;
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
    // Ensure persona exists
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": persona_name,
        "description": "Test persona",
        "prompt": "You are a test persona."
    });
    app.oneshot(put_json_auth("/personas", &body, token))
        .await
        .unwrap();

    // Create agent
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": agent_name,
        "persona": persona_name
    });
    app.oneshot(post_json_auth("/agents", &body, token))
        .await
        .unwrap();
}

/// Seed a texture in the brain.
async fn seed_texture(state: &Arc<ServiceState>, token: &str, texture_name: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": texture_name,
        "description": "Test texture",
        "prompt": "Reflect on this texture."
    });
    app.oneshot(put_json_auth("/textures", &body, token))
        .await
        .unwrap();
}

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
    let cognition: Record<CognitionId, Cognition> = serde_json::from_slice(&bytes).unwrap();
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
    let list: Vec<Record<CognitionId, Cognition>> = serde_json::from_slice(&bytes).unwrap();
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
    let list: Vec<Record<CognitionId, Cognition>> = serde_json::from_slice(&bytes).unwrap();
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
    let list: Vec<Record<CognitionId, Cognition>> = serde_json::from_slice(&bytes).unwrap();
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
    let list: Vec<Record<CognitionId, Cognition>> = serde_json::from_slice(&bytes).unwrap();
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
    let created: Record<CognitionId, Cognition> = serde_json::from_slice(&bytes).unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/cognitions/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let fetched: Record<CognitionId, Cognition> = serde_json::from_slice(&bytes).unwrap();
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
