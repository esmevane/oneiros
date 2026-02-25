use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use oneiros_db::Database;
use oneiros_model::*;
use oneiros_service::{ServiceState, projections, router};
use std::sync::Arc;
use tempfile::TempDir;
use tower::util::ServiceExt;

fn seed_tenant_and_actor(db: &Database) {
    let tenant_id = TenantId::new();

    let event = Events::Tenant(TenantEvents::TenantCreated(Tenant {
        id: tenant_id,
        name: TenantName::new("Test Tenant"),
    }));
    db.log_event(&event, projections::system::ALL).unwrap();

    let event = Events::Actor(ActorEvents::ActorCreated(Actor {
        id: ActorId::new(),
        tenant_id,
        name: ActorName::new("Test Actor"),
    }));
    db.log_event(&event, projections::system::ALL).unwrap();
}

fn setup() -> (TempDir, Arc<ServiceState>) {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("service.db");
    let db = Database::create(db_path).unwrap();
    seed_tenant_and_actor(&db);

    let state = Arc::new(ServiceState::new(db, temp.path().to_path_buf()));
    (temp, state)
}

fn get(uri: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

fn post_json(uri: &str, body: &serde_json::Value) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

#[tokio::test]
async fn health_returns_ok() {
    let (_temp, state) = setup();
    let app = router(state);

    let response = app.oneshot(get("/health")).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_brain_returns_created() {
    let (temp, state) = setup();
    let app = router(state);

    let body = serde_json::json!({ "name": "test-project" });
    let response = app.oneshot(post_json("/brains", &body)).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let info: BrainInfo = serde_json::from_slice(&bytes).unwrap();

    assert!(!info.entity.is_empty());
    assert!(
        !info.token.as_str().is_empty(),
        "should return a ticket token"
    );

    // Token should be decodable with correct claims
    let claims = info.token.decode().expect("token should be decodable");
    assert!(!claims.brain_id.is_empty(), "brain_id claim should be set");

    // Verify brain.db was created on disk
    let brain_path = temp.path().join("brains").join("test-project.db");
    assert!(
        brain_path.exists(),
        "brain.db should exist at {brain_path:?}"
    );
}

#[tokio::test]
async fn create_brain_conflict_on_duplicate() {
    let (_temp, state) = setup();

    let body = serde_json::json!({ "name": "duplicate-brain" });

    // First request
    let app = router(state.clone());
    let response = app.oneshot(post_json("/brains", &body)).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Second request — should conflict
    let app = router(state);
    let response = app.oneshot(post_json("/brains", &body)).await.unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn create_brain_fails_without_tenant() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("service.db");
    let db = Database::create(db_path).unwrap();
    // No tenant or actor seeded
    let state = Arc::new(ServiceState::new(db, temp.path().to_path_buf()));
    let app = router(state);

    let body = serde_json::json!({ "name": "orphan-brain" });
    let response = app.oneshot(post_json("/brains", &body)).await.unwrap();

    assert_eq!(response.status(), StatusCode::PRECONDITION_FAILED);
}

#[tokio::test]
async fn create_brain_returns_valid_ticket() {
    let (_temp, state) = setup();

    // Create a brain
    let app = router(state.clone());
    let body = serde_json::json!({ "name": "ticket-test" });
    let response = app.oneshot(post_json("/brains", &body)).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let info: BrainInfo = serde_json::from_slice(&bytes).unwrap();
    let token = info.token.as_str();

    // Use the returned token to list personas — should succeed with empty list
    let app = router(state);
    let request = Request::builder()
        .method(Method::GET)
        .uri("/personas")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
