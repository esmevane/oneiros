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

    let event = Events::Tenant(TenantEvents::TenantCreated(Identity::new(
        tenant_id,
        Tenant {
            name: TenantName::new("Test Tenant"),
        },
    )));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

    let event = Events::Actor(ActorEvents::ActorCreated(Identity::new(
        actor_id,
        Actor {
            tenant_id,
            name: ActorName::new("Test Actor"),
        },
    )));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

    Database::create_brain_db(brain_path).unwrap();

    let brain_id = BrainId::new();
    let event = Events::Brain(BrainEvents::BrainCreated(Identity::new(
        brain_id,
        Brain {
            tenant_id,
            name: BrainName::new("test-brain"),
            path: brain_path.to_path_buf(),
            status: BrainStatus::Active,
        },
    )));
    db.log_event(&event, projections::SYSTEM_PROJECTIONS)
        .unwrap();

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

/// Seed a nature into the brain so connections can reference it.
async fn seed_nature(state: &Arc<ServiceState>, token: &str, name: &str) {
    let app = router(state.clone());
    let body =
        serde_json::json!({ "name": name, "description": "Test nature.", "prompt": "Test." });
    let response = app
        .oneshot(put_json_auth("/natures", &body, token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_connection_returns_created() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "origin").await;

    let link_a = Link::new(&("test", "entity-a")).unwrap();
    let link_b = Link::new(&("test", "entity-b")).unwrap();

    let body = serde_json::json!({
        "nature": "origin",
        "from_link": link_a.to_string(),
        "to_link": link_b.to_string(),
    });

    let app = router(state);
    let response = app
        .oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let connection: Identity<ConnectionId, Connection> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(connection.nature, NatureName::new("origin"));
    assert_eq!(connection.from_link, link_a);
    assert_eq!(connection.to_link, link_b);
}

#[tokio::test]
async fn create_connection_with_unknown_nature_returns_not_found() {
    let (_temp, state, token) = setup();

    let link_a = Link::new(&("test", "entity-a")).unwrap();
    let link_b = Link::new(&("test", "entity-b")).unwrap();

    let body = serde_json::json!({
        "nature": "nonexistent",
        "from_link": link_a.to_string(),
        "to_link": link_b.to_string(),
    });

    let app = router(state);
    let response = app
        .oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_connections_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/connections", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Identity<ConnectionId, Connection>> = serde_json::from_slice(&bytes).unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_connections_after_create() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "origin").await;

    let link_a = Link::new(&("test", "entity-a")).unwrap();
    let link_b = Link::new(&("test", "entity-b")).unwrap();

    let body = serde_json::json!({
        "nature": "origin",
        "from_link": link_a.to_string(),
        "to_link": link_b.to_string(),
    });

    let app = router(state.clone());
    app.oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/connections", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Identity<ConnectionId, Connection>> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 1);
}

#[tokio::test]
async fn get_connection_not_found() {
    let (_temp, state, token) = setup();
    let fake_id = ConnectionId::new();
    let app = router(state);

    let response = app
        .oneshot(get_auth(&format!("/connections/{fake_id}"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn show_connection_by_id() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "context").await;

    let link_a = Link::new(&("test", "entity-a")).unwrap();
    let link_b = Link::new(&("test", "entity-b")).unwrap();

    let body = serde_json::json!({
        "nature": "context",
        "from_link": link_a.to_string(),
        "to_link": link_b.to_string(),
    });

    let app = router(state.clone());
    let response = app
        .oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: Identity<ConnectionId, Connection> = serde_json::from_slice(&bytes).unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/connections/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let fetched: Identity<ConnectionId, Connection> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.nature, NatureName::new("context"));
}

#[tokio::test]
async fn remove_connection_then_gone() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "origin").await;

    let link_a = Link::new(&("test", "entity-a")).unwrap();
    let link_b = Link::new(&("test", "entity-b")).unwrap();

    let body = serde_json::json!({
        "nature": "origin",
        "from_link": link_a.to_string(),
        "to_link": link_b.to_string(),
    });

    let app = router(state.clone());
    let response = app
        .oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: Identity<ConnectionId, Connection> = serde_json::from_slice(&bytes).unwrap();

    let app = router(state.clone());
    let response = app
        .oneshot(delete_auth(&format!("/connections/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/connections/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_connections_filters_by_nature() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "origin").await;
    seed_nature(&state, &token, "context").await;

    let link_a = Link::new(&("test", "entity-a")).unwrap();
    let link_b = Link::new(&("test", "entity-b")).unwrap();

    // Create one origin and one context connection.
    let body = serde_json::json!({
        "nature": "origin",
        "from_link": link_a.to_string(),
        "to_link": link_b.to_string(),
    });
    let app = router(state.clone());
    app.oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();

    let body = serde_json::json!({
        "nature": "context",
        "from_link": link_a.to_string(),
        "to_link": link_b.to_string(),
    });
    let app = router(state.clone());
    app.oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();

    // Filter by nature=origin should return 1.
    let app = router(state);
    let response = app
        .oneshot(get_auth("/connections?nature=origin", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Identity<ConnectionId, Connection>> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].nature, NatureName::new("origin"));
}

#[tokio::test]
async fn connection_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/connections")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn connection_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/connections", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_connection_by_link() {
    let (_temp, state, token) = setup();
    seed_nature(&state, &token, "origin").await;

    let link_a = Link::new(&("test", "entity-a")).unwrap();
    let link_b = Link::new(&("test", "entity-b")).unwrap();

    let body = serde_json::json!({
        "nature": "origin",
        "from_link": link_a.to_string(),
        "to_link": link_b.to_string(),
    });

    let app = router(state.clone());
    let response = app
        .oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: Identity<ConnectionId, Connection> = serde_json::from_slice(&bytes).unwrap();

    // Fetch by ID to get the link
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth(&format!("/connections/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let link = value["link"].as_str().unwrap().to_string();
    assert!(!link.is_empty());

    // Fetch by link
    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/connections/{link}"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let fetched: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(fetched["nature"], "origin");
    assert_eq!(fetched["link"], link);
}
