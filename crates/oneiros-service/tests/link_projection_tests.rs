use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use oneiros_db::Database;
use oneiros_link::*;
use oneiros_model::*;
use oneiros_service::*;
use rusqlite::params;
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

    // Verify link stored in DB and is parseable
    let conn = rusqlite::Connection::open(&brain_path).unwrap();
    let link: String = conn
        .query_row(
            "select link from agent where name = ?1",
            params!["governor.process"],
            |row| row.get(0),
        )
        .unwrap();

    assert!(!link.is_empty());
    assert!(link.parse::<Link>().is_ok());
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
    // Both links should be parseable
    assert!(link_before.parse::<Link>().is_ok());
    assert!(link_after.parse::<Link>().is_ok());
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
