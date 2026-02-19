use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use oneiros_db::Database;
use oneiros_model::*;
use oneiros_protocol::*;
use oneiros_service::*;
use std::sync::Arc;
use tempfile::TempDir;
use tower::util::ServiceExt;

fn seed_tenant_and_brain(db: &Database, brain_path: &std::path::Path) -> String {
    let tenant_id = TenantId::new();
    let actor_id = ActorId::new();

    let event = Events::Tenant(TenantEvents::TenantCreated(Tenant {
        tenant_id,
        name: TenantName::new("Test Tenant"),
    }));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

    let event = Events::Actor(ActorEvents::ActorCreated(Actor {
        tenant_id,
        actor_id,
        name: ActorName::new("Test Actor"),
    }));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

    Database::create_brain_db(brain_path).unwrap();

    let brain_id = BrainId::new();
    let event = Events::Brain(BrainEvents::BrainCreated(Brain {
        brain_id,
        tenant_id,
        name: BrainName::new("test-brain"),
        path: brain_path.to_path_buf(),
        status: BrainStatus::Active,
    }));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

    let token = Token::issue(TokenClaims {
        brain_id,
        tenant_id,
        actor_id,
    });

    let event = Events::Ticket(TicketEvents::TicketIssued(Ticket {
        ticket_id: TicketId::new(),
        token: token.clone(),
        created_by: actor_id,
    }));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

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

fn delete_auth(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::DELETE)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

async fn ensure_persona(state: &Arc<ServiceState>, token: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "expert",
        "description": "A domain expert",
        "prompt": "You are a domain expert."
    });
    app.oneshot(put_json_auth("/personas", &body, token))
        .await
        .unwrap();
}

#[tokio::test]
async fn create_agent_returns_created() {
    let (_temp, state, token) = setup();
    ensure_persona(&state, &token).await;

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
    let agent: Identity<AgentId, Agent> = serde_json::from_slice(&bytes).unwrap();
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
    ensure_persona(&state, &token).await;

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
    let list: Vec<Identity<AgentId, Agent>> = serde_json::from_slice(&bytes).unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_agents_after_create() {
    let (_temp, state, token) = setup();
    ensure_persona(&state, &token).await;

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
    let list: Vec<Identity<AgentId, Agent>> = serde_json::from_slice(&bytes).unwrap();
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
    ensure_persona(&state, &token).await;

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
    let agent: Identity<AgentId, Agent> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(agent.name, AgentName::new("architect"));
    assert_eq!(agent.persona, PersonaName::new("expert"));
}

#[tokio::test]
async fn update_agent() {
    let (_temp, state, token) = setup();
    ensure_persona(&state, &token).await;

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
    let agent: Identity<AgentId, Agent> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(agent.description.as_str(), "Version 2");
    assert_eq!(agent.prompt.as_str(), "Prompt v2");
}

#[tokio::test]
async fn update_nonexistent_agent() {
    let (_temp, state, token) = setup();
    ensure_persona(&state, &token).await;

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
    ensure_persona(&state, &token).await;

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
