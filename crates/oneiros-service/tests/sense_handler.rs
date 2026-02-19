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

fn post_auth(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
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

fn post_json_auth(uri: &str, body: &serde_json::Value, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

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

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let agent: Identity<AgentId, Agent> = serde_json::from_slice(&bytes).unwrap();
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
