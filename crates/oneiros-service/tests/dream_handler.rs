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
    db.log_event(&event, projections::SYSTEM).unwrap();

    let event = Events::Actor(ActorEvents::ActorCreated(Actor {
        id: actor_id,
        tenant_id,
        name: ActorName::new("Test Actor"),
    }));
    db.log_event(&event, projections::SYSTEM).unwrap();

    Database::create_brain_db(brain_path).unwrap();

    let brain_id = BrainId::new();
    let event = Events::Brain(BrainEvents::BrainCreated(Brain {
        id: brain_id,
        tenant_id,
        name: BrainName::new("test-brain"),
        status: BrainStatus::Active,
        path: brain_path.to_path_buf(),
    }));

    db.log_event(&event, projections::SYSTEM).unwrap();

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
    db.log_event(&event, projections::SYSTEM).unwrap();

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

/// Connections are scoped to an agent's entities (memories + experiences).
/// Each agent sees only the connections touching their own identity graph.
#[tokio::test]
async fn dream_scopes_connections_to_agent() {
    let (_temp, state, token) = setup();

    // Set up two agents with a shared persona.
    seed_agent(&state, &token, "agent-a", "process").await;
    seed_agent(&state, &token, "agent-b", "process").await;
    seed_texture(&state, &token, "working").await;
    seed_nature(&state, &token, "reference").await;

    // Create memories for each agent (memories anchor the graph).
    let mem_a = create_memory(&state, &token, "agent-a", "session", "Memory from A.").await;
    let mem_b = create_memory(&state, &token, "agent-b", "session", "Memory from B.").await;

    // Create cognitions for each agent.
    let cog_a = create_cognition(&state, &token, "agent-a", "working", "Thought from A.").await;
    let cog_b = create_cognition(&state, &token, "agent-b", "working", "Thought from B.").await;

    // Connect each agent's memory to their cognition.
    let _conn_a = create_connection(
        &state,
        &token,
        "reference",
        &Ref::memory(mem_a.id),
        &Ref::cognition(cog_a.id),
    )
    .await;

    let _conn_b = create_connection(
        &state,
        &token,
        "reference",
        &Ref::memory(mem_b.id),
        &Ref::cognition(cog_b.id),
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
    assert_eq!(context.connections[0].from_ref, Ref::memory(mem_a.id));

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
    assert_eq!(context.connections[0].from_ref, Ref::memory(mem_b.id));
}

async fn create_memory(
    state: &Arc<ServiceState>,
    token: &str,
    agent: &str,
    level: &str,
    content: &str,
) -> Memory {
    // Seed level if not yet seeded.
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": level,
        "description": "Test level",
        "prompt": "Test."
    });
    app.oneshot(put_json_auth("/levels", &body, token))
        .await
        .unwrap();

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
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

/// When connections exist, the dream should include cognitions that are
/// reachable via those connections — not ALL cognitions.
#[tokio::test]
async fn dream_collector_returns_connected_cognitions() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "dreamer", "process").await;
    seed_texture(&state, &token, "working").await;
    seed_nature(&state, &token, "reference").await;

    // Create a memory so connections are scoped to this agent.
    let mem = create_memory(&state, &token, "dreamer", "session", "Test memory.").await;

    // Create cognitions — one connected via the memory, one not.
    let connected_cog =
        create_cognition(&state, &token, "dreamer", "working", "Connected thought.").await;
    let _unconnected_cog =
        create_cognition(&state, &token, "dreamer", "working", "Unconnected thought.").await;

    // Connect the memory to one cognition.
    create_connection(
        &state,
        &token,
        "reference",
        &Ref::memory(mem.id),
        &Ref::cognition(connected_cog.id),
    )
    .await;

    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/dreamer", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    // Graph traversal is active (connections exist), so not all cognitions
    // are included — only connected + recent 20.
    // Both cognitions are recent (< 20), so both appear via the recent window.
    // The important thing: the collector used graph traversal, not full dump.
    assert!(!context.cognitions.is_empty());
    assert!(context.cognitions.iter().any(|c| c.id == connected_cog.id));

    // Connections should be present — scoped to this agent's entities.
    assert_eq!(context.connections.len(), 1);
}

/// With no connections at all, the dream falls back to ALL cognitions.
#[tokio::test]
async fn dream_collector_falls_back_for_empty_graph() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "sparse", "process").await;
    seed_texture(&state, &token, "working").await;

    // Create several cognitions but NO connections.
    for i in 0..5 {
        create_cognition(&state, &token, "sparse", "working", &format!("Thought {i}")).await;
    }

    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/sparse", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    // Fallback: ALL cognitions included.
    assert_eq!(context.cognitions.len(), 5);
    assert!(context.connections.is_empty());
}

/// Memories are always included in the dream regardless of graph state.
#[tokio::test]
async fn dream_collector_always_includes_all_memories() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "rememberer", "process").await;
    seed_texture(&state, &token, "working").await;

    // Create memories.
    let _mem1 = create_memory(&state, &token, "rememberer", "session", "First memory.").await;
    let _mem2 = create_memory(&state, &token, "rememberer", "session", "Second memory.").await;
    let _mem3 = create_memory(&state, &token, "rememberer", "session", "Third memory.").await;

    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/rememberer", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    // All memories always present — never filtered.
    assert_eq!(context.memories.len(), 3);
}

/// Recent cognitions appear in the dream even when not connected via the
/// graph, ensuring the agent has current-session context for continuity.
#[tokio::test]
async fn dream_collector_includes_recent_cognitions() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "oriented", "process").await;
    seed_texture(&state, &token, "working").await;
    seed_nature(&state, &token, "reference").await;

    // Create a memory so the collector can find connections.
    let mem = create_memory(&state, &token, "oriented", "session", "Anchor.").await;

    // Create a connected cognition (reachable via the graph).
    let connected =
        create_cognition(&state, &token, "oriented", "working", "Graph-connected.").await;
    create_connection(
        &state,
        &token,
        "reference",
        &Ref::memory(mem.id),
        &Ref::cognition(connected.id),
    )
    .await;

    // Create a recent cognition that is NOT connected.
    let recent =
        create_cognition(&state, &token, "oriented", "working", "Recent unconnected.").await;

    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/oriented", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    // Both the connected cognition and the recent one should be present.
    let ids: Vec<CognitionId> = context.cognitions.iter().map(|c| c.id).collect();
    assert!(
        ids.contains(&connected.id),
        "connected cognition should be in dream"
    );
    assert!(
        ids.contains(&recent.id),
        "recent cognition should be in dream"
    );
}
