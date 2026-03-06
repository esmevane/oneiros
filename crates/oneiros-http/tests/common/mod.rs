#![allow(dead_code)]

pub use axum::body::Body;
pub use axum::http::{Method, Request, StatusCode};
pub use http_body_util::BodyExt;
pub use oneiros_db::Database;
pub use oneiros_http::*;
pub use oneiros_model::*;
pub use std::sync::Arc;
pub use tempfile::TempDir;
pub use tower::util::ServiceExt;

// -- Core setup --

pub fn seed_tenant_and_brain(db: &Database, brain_path: &std::path::Path) -> String {
    let tenant_id = TenantId::new();
    let actor_id = ActorId::new();

    let event = Events::Tenant(TenantEvents::TenantCreated(Tenant {
        id: tenant_id,
        name: TenantName::new("Test Tenant"),
    }));
    db.log_event(&Event::create(event), projections::SYSTEM)
        .unwrap();

    let event = Events::Actor(ActorEvents::ActorCreated(Actor {
        id: actor_id,
        tenant_id,
        name: ActorName::new("Test Actor"),
    }));
    db.log_event(&Event::create(event), projections::SYSTEM)
        .unwrap();

    Database::create_brain_db(brain_path).unwrap();

    let brain_id = BrainId::new();
    let event = Events::Brain(BrainEvents::BrainCreated(Brain {
        id: brain_id,
        tenant_id,
        name: BrainName::new("test-brain"),
        status: BrainStatus::Active,
        path: brain_path.to_path_buf(),
    }));
    db.log_event(&Event::create(event), projections::SYSTEM)
        .unwrap();

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
    db.log_event(&Event::create(event), projections::SYSTEM)
        .unwrap();

    token.0
}

pub fn setup() -> (TempDir, Arc<ServiceState>, String) {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("service.db");
    let db = Database::create(db_path).unwrap();

    let brain_path = temp.path().join("brains").join("test-brain.db");
    std::fs::create_dir_all(brain_path.parent().unwrap()).unwrap();
    let token = seed_tenant_and_brain(&db, &brain_path);

    let state = Arc::new(ServiceState::new(db, temp.path().to_path_buf()));
    (temp, state, token)
}

// -- Request builders --

pub fn get(uri: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

pub fn get_auth(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

pub fn post_auth(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

pub fn post_json(uri: &str, body: &serde_json::Value) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

pub fn post_json_auth(uri: &str, body: &serde_json::Value, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

pub fn put_json_auth(uri: &str, body: &serde_json::Value, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::PUT)
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

pub fn delete_auth(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::DELETE)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

// -- Response helpers --

/// Parse response body, extracting from `{ "type": "...", "data": ... }` envelope
/// when present, falling back to raw deserialization for non-enveloped responses.
pub async fn body_json<T: serde::de::DeserializeOwned>(response: axum::response::Response) -> T {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    if let Some(data) = value.get("data") {
        serde_json::from_value(data.clone()).unwrap()
    } else {
        serde_json::from_value(value).unwrap()
    }
}

/// Parse response body as a raw bytes → T (no envelope extraction).
pub async fn body_bytes<T: serde::de::DeserializeOwned>(response: axum::response::Response) -> T {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

// -- Domain seeders --

pub async fn seed_agent(
    state: &Arc<ServiceState>,
    token: &str,
    agent_name: &str,
    persona_name: &str,
) {
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

pub async fn seed_texture(state: &Arc<ServiceState>, token: &str, name: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": name,
        "description": "Test texture",
        "prompt": "Test."
    });
    app.oneshot(put_json_auth("/textures", &body, token))
        .await
        .unwrap();
}

pub async fn seed_nature(state: &Arc<ServiceState>, token: &str, name: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": name,
        "description": "Test nature",
        "prompt": "Test."
    });
    app.oneshot(put_json_auth("/natures", &body, token))
        .await
        .unwrap();
}

pub async fn seed_level(state: &Arc<ServiceState>, token: &str, name: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": name,
        "description": "Test level",
        "prompt": "Test."
    });
    app.oneshot(put_json_auth("/levels", &body, token))
        .await
        .unwrap();
}

pub async fn seed_sensation(state: &Arc<ServiceState>, token: &str, name: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": name,
        "description": "Test sensation",
        "prompt": "Test."
    });
    app.oneshot(put_json_auth("/sensations", &body, token))
        .await
        .unwrap();
}

pub async fn ensure_persona(
    state: &Arc<ServiceState>,
    token: &str,
    name: &str,
    description: &str,
    prompt: &str,
) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": name,
        "description": description,
        "prompt": prompt,
    });
    app.oneshot(put_json_auth("/personas", &body, token))
        .await
        .unwrap();
}

// -- Factory helpers --

pub async fn create_cognition(
    state: &Arc<ServiceState>,
    token: &str,
    agent: &str,
    texture: &str,
    content: &str,
) -> Cognition {
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": agent,
        "texture": texture,
        "content": content
    });
    let response = app
        .oneshot(post_json_auth("/cognitions", &body, token))
        .await
        .unwrap();
    body_json(response).await
}

pub async fn create_connection(
    state: &Arc<ServiceState>,
    token: &str,
    nature: &str,
    from_ref: &Ref,
    to_ref: &Ref,
) -> Connection {
    let app = router(state.clone());
    let body = serde_json::json!({
        "nature": nature,
        "from_ref": serde_json::to_value(from_ref).unwrap(),
        "to_ref": serde_json::to_value(to_ref).unwrap(),
    });
    let response = app
        .oneshot(post_json_auth("/connections", &body, token))
        .await
        .unwrap();
    body_json(response).await
}

pub async fn create_memory(
    state: &Arc<ServiceState>,
    token: &str,
    agent: &str,
    level: &str,
    content: &str,
) -> Memory {
    // Seed level if not yet seeded (idempotent PUT).
    seed_level(state, token, level).await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": agent,
        "level": level,
        "content": content
    });
    let response = app
        .oneshot(post_json_auth("/memories", &body, token))
        .await
        .unwrap();
    body_json(response).await
}

pub async fn create_experience(
    state: &Arc<ServiceState>,
    token: &str,
    agent: &str,
    sensation: &str,
    description: &str,
) -> Experience {
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": agent,
        "sensation": sensation,
        "description": description
    });
    let response = app
        .oneshot(post_json_auth("/experiences", &body, token))
        .await
        .unwrap();
    body_json(response).await
}
