mod common;
use common::*;

fn seed_agent_in_brain(brain_path: &std::path::Path) {
    let brain_db = Database::open_brain(brain_path).unwrap();

    // Seed persona first (agent has FK to persona).
    let event = Events::Persona(PersonaEvents::PersonaSet(Persona::init(
        PersonaName::new("process"),
        "A process agent.",
        "You are a process agent.",
    )));
    brain_db.log_event(&event, projections::BRAIN).unwrap();

    let event = Events::Agent(AgentEvents::AgentCreated(Agent::init(
        "The governor agent.",
        "You are the governor.",
        AgentName::new("governor.process"),
        PersonaName::new("process"),
    )));
    brain_db.log_event(&event, projections::BRAIN).unwrap();
}

async fn get_dashboard(state: Arc<ServiceState>, uri: &str) -> (StatusCode, String) {
    let app = router(state);
    let request = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let text = String::from_utf8(body.to_vec()).unwrap();

    (status, text)
}

#[tokio::test]
async fn dashboard_returns_html() {
    let (_temp, state, _token) = setup();

    let (status, body) = get_dashboard(state, "/").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("<!doctype html>"), "expected HTML document");
    assert!(body.contains("test-brain"), "expected brain name in HTML");
}

#[tokio::test]
async fn dashboard_shows_agent_count() {
    let (temp, state, _token) = setup();

    let brain_path = temp.path().join("brains").join("test-brain.db");
    seed_agent_in_brain(&brain_path);

    let (status, body) = get_dashboard(state, "/").await;

    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("governor.process"),
        "expected agent name in HTML"
    );
}

#[tokio::test]
async fn dashboard_contains_sse_client() {
    let (_temp, state, _token) = setup();

    let (_, body) = get_dashboard(state, "/").await;

    assert!(
        body.contains("EventSource"),
        "expected SSE EventSource in inline JS"
    );
}

#[tokio::test]
async fn dashboard_handles_no_brains() {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("service.db");
    let db = Database::create(db_path).unwrap();
    let state = Arc::new(ServiceState::new(db, temp.path().to_path_buf()));

    let (status, body) = get_dashboard(state, "/").await;

    assert_eq!(status, StatusCode::OK);
    assert!(
        body.contains("No brains found"),
        "expected empty-state message"
    );
}
