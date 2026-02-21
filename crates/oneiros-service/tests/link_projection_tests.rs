use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use oneiros_db::Database;
use oneiros_link::*;
use oneiros_model::*;
use oneiros_protocol::*;
use oneiros_service::*;
use rusqlite::params;
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
    db.log_event(&event, projections::system::ALL).unwrap();

    let event = Events::Actor(ActorEvents::ActorCreated(Identity::new(
        actor_id,
        Actor {
            tenant_id,
            name: ActorName::new("Test Actor"),
        },
    )));
    db.log_event(&event, projections::system::ALL).unwrap();

    Database::create_brain_db(brain_path).unwrap();

    let brain_id = BrainId::new();
    let event = Events::Brain(BrainEvents::BrainCreated(Identity::new(
        brain_id,
        HasPath::new(
            brain_path,
            Brain {
                tenant_id,
                name: BrainName::new("test-brain"),
                status: BrainStatus::Active,
            },
        ),
    )));

    db.log_event(&event, projections::system::ALL).unwrap();

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
    db.log_event(&event, projections::system::ALL).unwrap();

    token.0
}

fn setup() -> (TempDir, Arc<ServiceState>, String, std::path::PathBuf) {
    let temp = TempDir::new().unwrap();
    let db_path = temp.path().join("service.db");
    let db = Database::create(db_path).unwrap();

    let brain_path = temp.path().join("brains").join("test-brain.db");
    std::fs::create_dir_all(brain_path.parent().unwrap()).unwrap();
    let token = seed_tenant_and_brain(&db, &brain_path);

    let state = Arc::new(ServiceState::new(db, temp.path().to_path_buf()));
    (temp, state, token, brain_path)
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

fn post_json_auth(uri: &str, body: &serde_json::Value, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

/// Name-keyed upsert: persona stores its link on creation.
#[tokio::test]
async fn persona_set_stores_link() {
    let (_temp, state, token, brain_path) = setup();
    let app = router(state);

    let body = serde_json::json!({
        "name": "expert",
        "description": "A domain expert",
        "prompt": "You are a domain expert."
    });

    let response = app
        .oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify link stored in DB
    let conn = rusqlite::Connection::open(&brain_path).unwrap();
    let link: String = conn
        .query_row(
            "select link from persona where name = ?1",
            params!["expert"],
            |row| row.get(0),
        )
        .unwrap();

    let expected = Persona::from("expert")
        .as_link()
        .unwrap()
        .to_link_string()
        .unwrap();

    assert_eq!(link, expected);
}

/// UUID-keyed create: agent stores its link on creation.
#[tokio::test]
async fn agent_created_stores_link() {
    let (_temp, state, token, brain_path) = setup();

    // Seed persona first
    let app = router(state.clone());
    let body = serde_json::json!({ "name": "process", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();

    // Create agent
    let app = router(state);
    let body = serde_json::json!({
        "name": "governor.process",
        "persona": "process",
        "description": "The governor",
        "prompt": "You are the governor."
    });

    let response = app
        .oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Verify link stored in DB
    let conn = rusqlite::Connection::open(&brain_path).unwrap();
    let link: String = conn
        .query_row(
            "select link from agent where name = ?1",
            params!["governor.process"],
            |row| row.get(0),
        )
        .unwrap();

    let expected = Agent {
        name: AgentName::new("governor.process"),
        persona: PersonaName::new("process"),
    }
    .as_link()
    .unwrap()
    .to_link_string()
    .unwrap();

    assert_eq!(link, expected);
}

/// Update recomputes: changing persona changes the link.
#[tokio::test]
async fn agent_updated_recomputes_link() {
    let (_temp, state, token, brain_path) = setup();

    // Seed two personas
    let app = router(state.clone());
    let body = serde_json::json!({ "name": "process", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "expert", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();

    // Create agent with process persona
    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "test.agent",
        "persona": "process",
        "description": "",
        "prompt": ""
    });
    app.oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();

    let conn = rusqlite::Connection::open(&brain_path).unwrap();
    let link_before: String = conn
        .query_row(
            "select link from agent where name = ?1",
            params!["test.agent"],
            |row| row.get(0),
        )
        .unwrap();

    // Update agent to expert persona
    let app = router(state);
    let body = serde_json::json!({
        "name": "test.agent",
        "persona": "expert",
        "description": "updated",
        "prompt": "updated"
    });
    app.oneshot(put_json_auth("/agents/test.agent", &body, &token))
        .await
        .unwrap();

    let link_after: String = conn
        .query_row(
            "select link from agent where name = ?1",
            params!["test.agent"],
            |row| row.get(0),
        )
        .unwrap();

    // Link should change because persona is part of agent identity
    assert_ne!(link_before, link_after);

    let expected = Agent {
        name: AgentName::new("test.agent"),
        persona: PersonaName::new("expert"),
    }
    .as_link()
    .unwrap()
    .to_link_string()
    .unwrap();

    assert_eq!(link_after, expected);
}

/// ID + content identity: cognition stores its link.
#[tokio::test]
async fn cognition_added_stores_link() {
    let (_temp, state, token, brain_path) = setup();

    // Seed persona, texture, and agent
    let app = router(state.clone());
    let body = serde_json::json!({ "name": "process", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/personas", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({ "name": "observation", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/textures", &body, &token))
        .await
        .unwrap();

    let app = router(state.clone());
    let body = serde_json::json!({
        "name": "test.agent",
        "persona": "process",
        "description": "",
        "prompt": ""
    });
    app.oneshot(post_json_auth("/agents", &body, &token))
        .await
        .unwrap();

    // Add a cognition
    let app = router(state);
    let body = serde_json::json!({
        "agent": "test.agent",
        "texture": "observation",
        "content": "The sky is blue today."
    });
    let response = app
        .oneshot(post_json_auth("/cognitions", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Verify link stored in DB — every cognition row should have a non-null link
    let conn = rusqlite::Connection::open(&brain_path).unwrap();
    let link: String = conn
        .query_row("select link from cognition limit 1", [], |row| row.get(0))
        .unwrap();

    // Link should be non-empty base64url
    assert!(!link.is_empty());
    // Link should be parseable back to a Link
    assert!(link.parse::<Link>().is_ok());
}

/// Self-link alongside from_link/to_link: connection stores its own entity link.
#[tokio::test]
async fn connection_created_stores_link() {
    let (_temp, state, token, brain_path) = setup();

    // Seed nature
    let app = router(state.clone());
    let body = serde_json::json!({ "name": "origin", "description": "", "prompt": "" });
    app.oneshot(put_json_auth("/natures", &body, &token))
        .await
        .unwrap();

    // Create a connection with arbitrary from/to links
    let app = router(state);
    let from_link = Link::new(&("texture", "observation")).unwrap().to_string();
    let to_link = Link::new(&("texture", "reflection")).unwrap().to_string();
    let body = serde_json::json!({
        "nature": "origin",
        "from_link": from_link,
        "to_link": to_link,
    });
    let response = app
        .oneshot(post_json_auth("/connections", &body, &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Verify the connection's own link is stored (distinct from from_link/to_link)
    let conn = rusqlite::Connection::open(&brain_path).unwrap();
    let (entity_link, stored_from, stored_to): (String, String, String) = conn
        .query_row(
            "select link, from_link, to_link from connection limit 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .unwrap();

    // Entity link should be distinct from the edge endpoints
    assert_ne!(entity_link, stored_from);
    assert_ne!(entity_link, stored_to);
    // Entity link should be parseable
    assert!(entity_link.parse::<Link>().is_ok());
    // Edge endpoints should match what we sent
    assert_eq!(stored_from, from_link);
    assert_eq!(stored_to, to_link);
}
