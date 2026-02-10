use std::sync::Arc;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use oneiros_client::BrainInfo;
use oneiros_db::Database;
use oneiros_model::{BrainStatus, Events, Id, Label, Tenant, TenantEvents, projections};
use oneiros_service::{ServiceState, router};
use tempfile::TempDir;
use tower::util::ServiceExt;

fn seed_tenant(db: &Database) {
    let event = Events::Tenant(TenantEvents::TenantCreated(Tenant {
        tenant_id: Id::new(),
        name: Label::new("Test Tenant"),
    }));

    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();
}

fn setup() -> (TempDir, Arc<ServiceState>) {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("service.db");
    let db = Database::create(db_path).unwrap();
    seed_tenant(&db);

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

    assert_eq!(info.name, Label::new("test-project"));
    assert!(matches!(info.status, BrainStatus::Active));
    assert!(!info.id.is_empty());

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
    // No tenant seeded
    let state = Arc::new(ServiceState::new(db, temp.path().to_path_buf()));
    let app = router(state);

    let body = serde_json::json!({ "name": "orphan-brain" });
    let response = app.oneshot(post_json("/brains", &body)).await.unwrap();

    assert_eq!(response.status(), StatusCode::PRECONDITION_FAILED);
}
