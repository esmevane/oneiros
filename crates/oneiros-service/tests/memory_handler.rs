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

/// Seed a level in the brain.
async fn seed_level(state: &Arc<ServiceState>, token: &str, level_name: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": level_name,
        "description": "Test level",
        "prompt": "Guidance for this level."
    });
    app.oneshot(put_json_auth("/levels", &body, token))
        .await
        .unwrap();
}

#[tokio::test]
async fn add_memory_returns_created() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_level(&state, &token, "core").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "The system uses event sourcing for all state changes."
    });

    let response = app
        .oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let memory: Identity<MemoryId, Memory> = serde_json::from_slice(&bytes).unwrap();
    assert!(!memory.id.is_empty());
    assert_eq!(memory.level, LevelName::new("core"));
    assert_eq!(
        memory.content.as_str(),
        "The system uses event sourcing for all state changes."
    );
}

#[tokio::test]
async fn add_memory_requires_existing_agent() {
    let (_temp, state, token) = setup();
    seed_level(&state, &token, "core").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "nonexistent",
        "level": "core",
        "content": "This should fail."
    });

    let response = app
        .oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn add_memory_requires_existing_level() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;

    let app = router(state);
    let body = serde_json::json!({
        "agent": "architect",
        "level": "nonexistent",
        "content": "This should fail."
    });

    let response = app
        .oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_memories_empty() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let response = app.oneshot(get_auth("/memories", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Identity<MemoryId, Memory>> = serde_json::from_slice(&bytes).unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_memories_after_add() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_level(&state, &token, "core").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "First memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "Second memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app.oneshot(get_auth("/memories", &token)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Identity<MemoryId, Memory>> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn list_memories_filtered_by_agent() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_agent(&state, &token, "editor", "expert").await;
    seed_level(&state, &token, "core").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "Architect memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "editor",
        "level": "core",
        "content": "Editor memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth("/memories?agent=architect", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Identity<MemoryId, Memory>> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].content.as_str(), "Architect memory.");
}

#[tokio::test]
async fn list_memories_filtered_by_level() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_level(&state, &token, "core").await;
    seed_level(&state, &token, "active").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "A core memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "active",
        "content": "An active memory."
    });
    app.oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth("/memories?level=active", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let list: Vec<Identity<MemoryId, Memory>> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].content.as_str(), "An active memory.");
}

#[tokio::test]
async fn get_memory_not_found() {
    let (_temp, state, token) = setup();
    let app = router(state);

    let fake_id = MemoryId::new();
    let response = app
        .oneshot(get_auth(&format!("/memories/{fake_id}"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_memory_by_id() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_level(&state, &token, "core").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "A notable memory."
    });
    let response = app
        .oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: Identity<MemoryId, Memory> = serde_json::from_slice(&bytes).unwrap();

    let app = router(state);
    let response = app
        .oneshot(get_auth(&format!("/memories/{}", created.id), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let fetched: Identity<MemoryId, Memory> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.content.as_str(), "A notable memory.");
}

#[tokio::test]
async fn memory_request_without_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/memories")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn memory_request_with_invalid_token_returns_unauthorized() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let response = app
        .oneshot(get_auth("/memories", "bogus-token-value"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_memory_by_link() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "architect", "expert").await;
    seed_level(&state, &token, "core").await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "architect",
        "level": "core",
        "content": "A memory resolved by link."
    });
    let response = app
        .oneshot(post_json_auth("/memories", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: Identity<MemoryId, Memory> = serde_json::from_slice(&bytes).unwrap();

    // Fetch by ID to get the link
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth(&format!("/memories/{}", created.id), &token))
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
        .oneshot(get_auth(&format!("/memories/{link}"), &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let fetched: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(fetched["content"], "A memory resolved by link.");
    assert_eq!(fetched["link"], link);
}
