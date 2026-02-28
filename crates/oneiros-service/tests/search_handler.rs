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

async fn seed_agent(state: &Arc<ServiceState>, token: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "tester",
        "description": "Test persona",
        "prompt": "You are a test persona."
    });
    app.oneshot(put_json_auth("/personas", &body, token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "tester",
        "description": "Test texture",
        "prompt": "Test."
    });
    app.oneshot(put_json_auth("/textures", &body, token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "searcher", "persona": "tester" });
    app.oneshot(post_json_auth("/agents", &body, token))
        .await
        .unwrap();
}

#[tokio::test]
async fn search_finds_cognition_content() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token).await;

    // Add a cognition
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "searcher",
        "texture": "tester",
        "content": "The quick brown fox jumps over the lazy dog"
    });
    let response = app
        .oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Search for it
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?q=fox", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let results: SearchResults = serde_json::from_slice(&body).unwrap();

    assert_eq!(results.query, "fox");
    assert_eq!(results.results.len(), 1);
    assert_eq!(results.results[0].kind.as_str(), "cognition-content");
    assert!(results.results[0].content.as_str().contains("fox"));
}

#[tokio::test]
async fn search_returns_empty_for_no_match() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token).await;

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "searcher",
        "texture": "tester",
        "content": "Hello world"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?q=nonexistent", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let results: SearchResults = serde_json::from_slice(&body).unwrap();

    assert!(results.results.is_empty());
}

#[tokio::test]
async fn search_finds_agent_description() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token).await;

    // Update agent with a searchable description
    let app = router(state.clone());
    let body = serde_json::json!({
        "persona": "tester",
        "description": "A specialized quantum computing researcher",
        "prompt": "Think about qubits."
    });
    let response = app
        .oneshot(put_json_auth("/agents/searcher", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Search for the description
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?q=quantum", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let results: SearchResults = serde_json::from_slice(&body).unwrap();

    assert!(!results.results.is_empty());
    let kinds: Vec<&str> = results.results.iter().map(|r| r.kind.as_str()).collect();
    assert!(kinds.contains(&"agent-description"));
}

#[tokio::test]
async fn search_finds_persona_content() {
    let (_temp, state, token) = setup();

    // Create persona with searchable description
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "philosopher",
        "description": "Contemplates epistemological foundations",
        "prompt": "Think deeply about knowledge."
    });
    let response = app
        .oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Search for the description
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?q=epistemological", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let results: SearchResults = serde_json::from_slice(&body).unwrap();

    assert_eq!(results.results.len(), 1);
    assert_eq!(results.results[0].kind.as_str(), "persona-description");
}

#[tokio::test]
async fn search_across_multiple_entity_types() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token).await;

    // Add cognition with "architecture"
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "searcher",
        "texture": "tester",
        "content": "The architecture of this system is elegant"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Update agent description with "architecture"
    let app = router(state.clone());
    let body = serde_json::json!({
        "persona": "tester",
        "description": "An architecture expert agent",
        "prompt": "Think about systems."
    });
    app.oneshot(put_json_auth("/agents/searcher", &body, &token))
        .await
        .unwrap();

    // Search — should find both
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?q=architecture", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let results: SearchResults = serde_json::from_slice(&body).unwrap();

    assert!(results.results.len() >= 2);
    let kinds: Vec<&str> = results.results.iter().map(|r| r.kind.as_str()).collect();
    assert!(kinds.contains(&"cognition-content"));
    assert!(kinds.contains(&"agent-description"));
}

async fn seed_second_agent(state: &Arc<ServiceState>, token: &str) {
    let app = router(state.clone());
    let body = serde_json::json!({ "name": "other-agent", "persona": "tester" });
    app.oneshot(post_json_auth("/agents", &body, token))
        .await
        .unwrap();
}

#[tokio::test]
async fn search_scoped_to_agent_returns_matching() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token).await;
    seed_second_agent(&state, &token).await;

    // Add cognition for searcher
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "searcher",
        "texture": "tester",
        "content": "Quantum mechanics is fundamental"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Add cognition for other-agent with same keyword
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "other-agent",
        "texture": "tester",
        "content": "Quantum computing advances rapidly"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Search WITHOUT agent filter — should find both
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?q=quantum", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let all_results: SearchResults = serde_json::from_slice(&body).unwrap();
    assert_eq!(all_results.results.len(), 2);

    // Search WITH agent filter — should find only searcher's
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?q=quantum&agent=searcher", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let filtered_results: SearchResults = serde_json::from_slice(&body).unwrap();
    assert_eq!(filtered_results.results.len(), 1);
    assert!(
        filtered_results.results[0]
            .content
            .as_str()
            .contains("mechanics")
    );
}

#[tokio::test]
async fn search_without_agent_returns_all() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token).await;
    seed_second_agent(&state, &token).await;

    // Add cognitions for both agents
    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "searcher",
        "texture": "tester",
        "content": "Exploring neural pathways"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "agent": "other-agent",
        "texture": "tester",
        "content": "Neural network architectures"
    });
    app.oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();

    // Search without agent filter
    let app = router(state.clone());
    let response = app
        .oneshot(get_auth("/search?q=neural", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let results: SearchResults = serde_json::from_slice(&body).unwrap();

    // Both agents' cognitions should appear
    assert_eq!(results.results.len(), 2);
}
