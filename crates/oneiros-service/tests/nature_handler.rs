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

    let event = Events::Tenant(TenantEvents::TenantCreated(Tenant {
        id: tenant_id,
        name: TenantName::new("Test Tenant"),
    }));
    db.log_event(&event, projections::system::ALL).unwrap();

    let event = Events::Actor(ActorEvents::ActorCreated(Actor {
        id: actor_id,
        tenant_id,
        name: ActorName::new("Test Actor"),
    }));
    db.log_event(&event, projections::system::ALL).unwrap();

    Database::create_brain_db(brain_path).unwrap();

    let brain_id = BrainId::new();
    let event = Events::Brain(BrainEvents::BrainCreated(Brain {
        id: brain_id,
        tenant_id,
        name: BrainName::new("test-brain"),
        status: BrainStatus::Active,
        path: brain_path.to_path_buf(),
    }));

    db.log_event(&event, projections::system::ALL).unwrap();

    let token = Token::issue(TokenClaims {
        brain_id,
        tenant_id,
        actor_id,
    });

    let event = Events::Ticket(TicketEvents::TicketIssued(Ticket {
        id: TicketId::new(),
        token: token.clone(),
        created_by: actor_id,
    }));
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

#[tokio::test]
async fn set_nature_returns_ok() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let body = serde_json::json!({
        "name": "origin",
        "description": "The causal root of a connection.",
        "prompt": "Trace the origin of this relationship."
    });

    let response = app
        .oneshot(put_json_auth("/natures", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let info: Nature = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(info.name, NatureName::new("origin"));
    assert_eq!(
        info.description.as_str(),
        "The causal root of a connection."
    );
    assert_eq!(
        info.prompt.as_str(),
        "Trace the origin of this relationship."
    );
}

#[tokio::test]
async fn set_nature_is_idempotent() {
    let (_temp, state, token) = setup();

    let body = serde_json::json!({
        "name": "context",
        "description": "Version 1",
        "prompt": "Prompt v1"
    });
    let app = router(state.clone());
    let response = app
        .oneshot(put_json_auth("/natures", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body2 = serde_json::json!({
        "name": "context",
        "description": "Version 2",
        "prompt": "Prompt v2"
    });
    let app = router(state.clone());
    let response = app
        .oneshot(put_json_auth("/natures", &body2, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let app = router(state);
    let response = app
        .oneshot(get_auth("/natures/context", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let info: Nature = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(info.description.as_str(), "Version 2");
    assert_eq!(info.prompt.as_str(), "Prompt v2");
}

#[tokio::test]
async fn list_natures_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/natures", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Nature> = serde_json::from_slice(&bytes).unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_natures_after_set() {
    let (_temp, state, token) = setup();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "origin", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/natures", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "context", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/natures", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/natures", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Nature> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn get_nature_not_found() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/natures/nonexistent", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn remove_nature_then_gone() {
    let (_temp, state, token) = setup();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "ephemeral", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/natures", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let response = app
        .oneshot(delete_auth("/natures/ephemeral", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let app = router(state);
    let response = app
        .oneshot(get_auth("/natures/ephemeral", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn nature_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/natures")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn nature_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/natures", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
