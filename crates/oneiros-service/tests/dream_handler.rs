use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
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

fn post_auth(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
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

async fn seed_texture(state: &Arc<ServiceState>, token: &str, name: &str) {
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

async fn seed_nature(state: &Arc<ServiceState>, token: &str, name: &str) {
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

async fn create_cognition(
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
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

async fn create_connection(
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
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn dream_returns_agent_context() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "governor", "process").await;
    seed_texture(&state, &token, "working").await;

    let _cog = create_cognition(&state, &token, "governor", "working", "A test thought.").await;

    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/governor", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(context.agent.name, AgentName::new("governor"));
    assert_eq!(context.cognitions.len(), 1);
}

#[tokio::test]
async fn dream_scopes_connections_to_agent() {
    let (_temp, state, token) = setup();

    // Set up two agents with a shared persona.
    seed_agent(&state, &token, "agent-a", "process").await;
    seed_agent(&state, &token, "agent-b", "process").await;
    seed_texture(&state, &token, "working").await;
    seed_nature(&state, &token, "reference").await;

    // Create cognitions for each agent.
    let cog_a = create_cognition(&state, &token, "agent-a", "working", "Thought from A.").await;
    let cog_b = create_cognition(&state, &token, "agent-b", "working", "Thought from B.").await;

    // Create a connection between agent-a's cognitions (belongs to agent-a's graph).
    let cog_a2 = create_cognition(
        &state,
        &token,
        "agent-a",
        "working",
        "Another thought from A.",
    )
    .await;
    let _conn_a = create_connection(
        &state,
        &token,
        "reference",
        &Ref::cognition(cog_a.id),
        &Ref::cognition(cog_a2.id),
    )
    .await;

    // Create a connection between agent-b's cognitions (belongs to agent-b's graph).
    let cog_b2 = create_cognition(
        &state,
        &token,
        "agent-b",
        "working",
        "Another thought from B.",
    )
    .await;
    let _conn_b = create_connection(
        &state,
        &token,
        "reference",
        &Ref::cognition(cog_b.id),
        &Ref::cognition(cog_b2.id),
    )
    .await;

    // Dream agent-a: should see only agent-a's connection.
    let app = router(state.clone());
    let response = app
        .oneshot(post_auth("/dream/agent-a", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(
        context.connections.len(),
        1,
        "agent-a should see 1 connection, not both"
    );
    assert_eq!(context.connections[0].from_ref, Ref::cognition(cog_a.id));

    // Dream agent-b: should see only agent-b's connection.
    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/agent-b", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(
        context.connections.len(),
        1,
        "agent-b should see 1 connection, not both"
    );
    assert_eq!(context.connections[0].from_ref, Ref::cognition(cog_b.id));
}
