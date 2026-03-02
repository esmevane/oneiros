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

async fn ensure_persona(state: &Arc<ServiceState>, token: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "process",
        "description": "A process agent",
        "prompt": "You are a process agent."
    });
    app.oneshot(put_json_auth("/personas", &body, token))
        .await
        .unwrap();
}

#[tokio::test]
async fn activity_endpoint_returns_sse_content_type() {
    let (_temp, state, _token) = setup();
    let app = router(state);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/activity")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(
        content_type.contains("text/event-stream"),
        "expected SSE content type, got: {content_type}"
    );
}

#[tokio::test]
async fn broadcast_channel_receives_events_from_handlers() {
    let (_temp, state, token) = setup();

    // Seed the persona so agent creation succeeds.
    ensure_persona(&state, &token).await;

    // Subscribe to the broadcast channel before triggering events.
    let mut rx = state.subscribe();

    // Create an agent, which triggers a broadcast.
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "test-agent",
        "persona": "process"
    });
    let response = app
        .oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // The agent creation event should appear on the broadcast channel.
    let event: Events = rx.recv().await.unwrap();
    let json = serde_json::to_string(&event).unwrap();
    assert!(
        json.contains("agent-created"),
        "expected agent-created event, got: {json}"
    );
}

#[tokio::test]
async fn sse_stream_receives_broadcast_events() {
    let (_temp, state, _token) = setup();

    // Get the SSE response (streaming body).
    let app = router(state.clone());
    let request = Request::builder()
        .method(Method::GET)
        .uri("/activity")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let mut body = response.into_body();

    // Inject an event directly via the broadcast channel.
    let test_event = Events::Lifecycle(LifecycleEvents::Woke {
        name: AgentName::new("test-agent"),
    });
    state.broadcast(test_event);

    // Read the first frame from the SSE stream.
    let frame = tokio::time::timeout(std::time::Duration::from_secs(2), body.frame())
        .await
        .expect("timed out waiting for SSE frame")
        .expect("stream ended")
        .expect("frame error");

    let data = frame.into_data().expect("expected data frame");
    let text = String::from_utf8(data.to_vec()).unwrap();

    assert!(
        text.contains("test-agent"),
        "SSE frame should contain the event data, got: {text}"
    );
}

#[tokio::test]
async fn full_tcp_pipeline_serves_and_responds() {
    let (_temp, state, token) = setup();

    // Start a real TCP server on an ephemeral port.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let app = router(state.clone());
    let server = tokio::spawn(async move {
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    });

    // Give the server a moment to start.
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Use the real TCP client to make an API call.
    let client = oneiros_client::Client::new(addr);
    client.health().await.unwrap();

    // Seed a persona so agent creation succeeds.
    let token = Token(token);
    client
        .set_persona(
            &token,
            Persona {
                name: PersonaName::new("process"),
                description: Description::new("A process agent"),
                prompt: Prompt::new("You are a process agent."),
            },
        )
        .await
        .unwrap();

    // Create an agent through the TCP transport.
    let agent = client
        .create_agent(
            &token,
            CreateAgentRequest {
                name: AgentName::new("tcp-agent"),
                persona: PersonaName::new("process"),
                description: Description::default(),
                prompt: Prompt::default(),
            },
        )
        .await
        .unwrap();

    assert_eq!(agent.name.as_str(), "tcp-agent");

    server.abort();
}
