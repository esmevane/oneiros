use axum::body::Body;
use axum::http::{Request, StatusCode};
use rusqlite::Connection;
use tower::ServiceExt;

use crate::*;

// ── Helpers ───────────────────────────────────────────────────────

/// All project-scoped projections in registration order.
static PROJECTIONS: &[&[Projection]] = &[
    LevelProjections.all(),
    TextureProjections.all(),
    SensationProjections.all(),
    NatureProjections.all(),
    PersonaProjections.all(),
    UrgeProjections.all(),
    AgentProjections.all(),
    CognitionProjections.all(),
    MemoryProjections.all(),
    ExperienceProjections.all(),
    ConnectionProjections.all(),
    PressureProjections.all(),
    StorageProjections.all(),
    SearchProjections.all(),
];

fn project_ctx() -> ProjectContext {
    let conn = Connection::open_in_memory().expect("open db");
    migrations::migrate_project(&conn).expect("migrate");
    ProjectContext::new(conn, PROJECTIONS)
}

fn seed_persona(ctx: &ProjectContext) {
    PersonaService::set(
        ctx,
        Persona {
            name: "test-persona".into(),
            description: "A test persona".into(),
            prompt: "You are a test.".into(),
        },
    )
    .unwrap();
}

fn seed_agent(ctx: &ProjectContext) {
    AgentService::create(
        ctx,
        "gov".into(),
        "test-persona".into(),
        "Governor".into(),
        "You govern.".into(),
    )
    .unwrap();
}

// ── Vocabulary domain tests ───────────────────────────────────────

#[test]
fn level_crud() {
    let ctx = project_ctx();

    LevelService::set(
        &ctx,
        Level {
            name: "working".into(),
            description: "Active".into(),
            prompt: "".into(),
        },
    )
    .unwrap();
    assert!(matches!(
        LevelService::get(&ctx, "working").unwrap(),
        LevelResponse::LevelDetails(_)
    ));

    match LevelService::list(&ctx).unwrap() {
        LevelResponse::Levels(levels) => assert_eq!(levels.len(), 1),
        other => panic!("Expected Listed, got {other:?}"),
    }

    LevelService::remove(&ctx, "working").unwrap();
    assert!(LevelService::get(&ctx, "working").is_err());
}

#[test]
fn persona_crud() {
    let ctx = project_ctx();

    PersonaService::set(
        &ctx,
        Persona {
            name: "process".into(),
            description: "Process agents".into(),
            prompt: "".into(),
        },
    )
    .unwrap();
    assert!(matches!(
        PersonaService::get(&ctx, "process").unwrap(),
        PersonaResponse::PersonaDetails(_)
    ));
}

// ── Entity domain tests ──────────────────────────────────────────

#[test]
fn agent_create_and_get() {
    let ctx = project_ctx();
    seed_persona(&ctx);

    let resp = AgentService::create(
        &ctx,
        "governor".into(),
        "test-persona".into(),
        "The governor".into(),
        "You govern.".into(),
    )
    .unwrap();
    assert!(matches!(resp, AgentResponse::AgentCreated(_)));

    match AgentService::get(&ctx, "governor.test-persona").unwrap() {
        AgentResponse::AgentDetails(a) => {
            assert_eq!(a.name, AgentName::new("governor.test-persona"));
            assert_eq!(a.persona, PersonaName::new("test-persona"));
        }
        other => panic!("Expected AgentDetails, got {other:?}"),
    }
}

#[test]
fn agent_persona_validation() {
    let ctx = project_ctx();

    let result = AgentService::create(
        &ctx,
        "gov".into(),
        "nonexistent".into(),
        "".into(),
        "".into(),
    );
    assert!(matches!(result, Err(AgentError::PersonaNotFound(_))));
}

#[test]
fn agent_name_conflict() {
    let ctx = project_ctx();
    seed_persona(&ctx);

    AgentService::create(
        &ctx,
        "gov".into(),
        "test-persona".into(),
        "".into(),
        "".into(),
    )
    .unwrap();
    let result = AgentService::create(
        &ctx,
        "gov".into(),
        "test-persona".into(),
        "".into(),
        "".into(),
    );
    assert!(matches!(result, Err(AgentError::Conflict(_))));
}

#[test]
fn cognition_add_and_list() {
    let ctx = project_ctx();
    seed_persona(&ctx);
    seed_agent(&ctx);

    let resp = CognitionService::add(
        &ctx,
        "gov".into(),
        "observation".into(),
        "Something interesting".into(),
    )
    .unwrap();
    assert!(matches!(
        resp,
        CognitionResponse::CognitionAdded(CognitionAddedResult { .. })
    ));

    match CognitionService::list(&ctx, Some("gov"), None).unwrap() {
        CognitionResponse::Cognitions(cogs) => assert_eq!(cogs.len(), 1),
        other => panic!("Expected Cognitions, got {other:?}"),
    }
}

// ── Typed event tests ────────────────────────────────────────────

#[test]
fn broadcast_events_are_typed() {
    let ctx = project_ctx();
    seed_persona(&ctx);

    let mut sub = ctx.subscribe();

    LevelService::set(
        &ctx,
        Level {
            name: "working".into(),
            description: "".into(),
            prompt: "".into(),
        },
    )
    .unwrap();
    let event = sub.try_recv().unwrap();
    assert_eq!(event.event_type, "level-set");
    assert!(matches!(
        event.data,
        Events::Level(LevelEvents::LevelSet(_))
    ));

    AgentService::create(
        &ctx,
        "gov".into(),
        "test-persona".into(),
        "".into(),
        "".into(),
    )
    .unwrap();
    let event = sub.try_recv().unwrap();
    assert_eq!(event.event_type, "agent-created");
    assert!(matches!(
        event.data,
        Events::Agent(AgentEvents::AgentCreated(_))
    ));
}

// ── Replay test ──────────────────────────────────────────────────

#[test]
fn replay_reconstructs_read_models() {
    let ctx = project_ctx();

    // Seed data across multiple domains
    LevelService::set(
        &ctx,
        Level {
            name: "working".into(),
            description: "Active".into(),
            prompt: "".into(),
        },
    )
    .unwrap();
    LevelService::set(
        &ctx,
        Level {
            name: "session".into(),
            description: "Session".into(),
            prompt: "".into(),
        },
    )
    .unwrap();
    seed_persona(&ctx);
    seed_agent(&ctx);
    CognitionService::add(
        &ctx,
        "gov.test-persona".into(),
        "observation".into(),
        "Test thought".into(),
    )
    .unwrap();

    // Verify read models before replay
    match LevelService::list(&ctx).unwrap() {
        LevelResponse::Levels(levels) => assert_eq!(levels.len(), 2),
        other => panic!("Expected Listed, got {other:?}"),
    }
    assert!(matches!(
        AgentService::get(&ctx, "gov.test-persona").unwrap(),
        AgentResponse::AgentDetails(_)
    ));

    // Replay — this resets all projections and re-applies all events
    ctx.with_db(|conn| event::repo::replay(conn, PROJECTIONS).unwrap());

    // Read models should be identical after replay
    match LevelService::list(&ctx).unwrap() {
        LevelResponse::Levels(levels) => assert_eq!(levels.len(), 2),
        other => panic!("Expected Listed after replay, got {other:?}"),
    }
    match AgentService::get(&ctx, "gov.test-persona").unwrap() {
        AgentResponse::AgentDetails(a) => assert_eq!(a.name, AgentName::new("gov.test-persona")),
        other => panic!("Expected AgentDetails after replay, got {other:?}"),
    }
    match CognitionService::list(&ctx, Some("gov.test-persona"), None).unwrap() {
        CognitionResponse::Cognitions(cogs) => assert_eq!(cogs.len(), 1),
        other => panic!("Expected Cognitions after replay, got {other:?}"),
    }
}

// ── HTTP collector tests ──────────────────────────────────────────

#[tokio::test]
async fn http_serves_multiple_domains() {
    let ctx = project_ctx();
    seed_persona(&ctx);
    let app = crate::http::project_router(ctx);

    // Set a level
    let req: Request<Body> = Request::builder()
        .method("PUT")
        .uri("/levels/working")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"name":"working","description":"Active","prompt":""}"#,
        ))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Create an agent
    let req: Request<Body> = Request::builder()
        .method("POST")
        .uri("/agents")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"name":"gov","persona":"test-persona","description":"Gov","prompt":""}"#,
        ))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Read both back
    let req: Request<Body> = Request::builder()
        .uri("/levels/working")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let req: Request<Body> = Request::builder()
        .uri("/agents/gov.test-persona")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn http_not_found() {
    let ctx = project_ctx();
    let app = crate::http::project_router(ctx);

    let req: Request<Body> = Request::builder()
        .uri("/agents/nope")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let req: Request<Body> = Request::builder()
        .uri("/levels/nope")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ── Full integration ──────────────────────────────────────────────

#[tokio::test]
async fn full_integration() {
    let ctx = project_ctx();
    seed_persona(&ctx);
    let app = crate::http::project_router(ctx.clone());

    // Subscribe AFTER seeding so we only see the agent event
    let mut sub = ctx.subscribe();

    // Create agent via HTTP
    let req: Request<Body> = Request::builder()
        .method("POST")
        .uri("/agents")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"name":"gov","persona":"test-persona","description":"Gov","prompt":""}"#,
        ))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Read back via HTTP
    let req: Request<Body> = Request::builder()
        .uri("/agents/gov.test-persona")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Broadcast received a typed agent-created event
    let agent_event = sub.try_recv().unwrap();
    assert_eq!(agent_event.event_type, "agent-created");
    assert!(matches!(
        agent_event.data,
        Events::Agent(AgentEvents::AgentCreated(_))
    ));
}

// ── Event serialization round-trip ───────────────────────────────

#[test]
fn events_serialize_and_deserialize() {
    let event = Events::Level(LevelEvents::LevelSet(Level {
        name: "test".into(),
        description: "desc".into(),
        prompt: "p".into(),
    }));

    // Serialize
    let json = serde_json::to_string(&event).unwrap();
    assert!(json.contains(r#""type":"level-set""#));

    // Deserialize
    let back: Events = serde_json::from_str(&json).unwrap();
    assert!(matches!(back, Events::Level(LevelEvents::LevelSet(_))));

    // Unknown events deserialize to Unknown
    let unknown_json = r#"{"type":"future-event","data":{"x":1}}"#;
    let unknown: Events = serde_json::from_str(unknown_json).unwrap();
    assert!(matches!(unknown, Events::Unknown(_)));
}

// ── Search projection test ───────────────────────────────────────

#[test]
fn search_indexes_across_domains() {
    let ctx = project_ctx();
    seed_persona(&ctx);
    seed_agent(&ctx);

    // Add cognitions
    CognitionService::add(
        &ctx,
        "gov".into(),
        "observation".into(),
        "The architecture is clean".into(),
    )
    .unwrap();
    CognitionService::add(
        &ctx,
        "gov".into(),
        "working".into(),
        "Working on typed events".into(),
    )
    .unwrap();

    // Search should find them
    match SearchService::search(&ctx, "architecture", None).unwrap() {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }

    // Agent itself should be indexed too (from seed_agent)
    match SearchService::search(&ctx, "Governor", None).unwrap() {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }

    // Search with agent filter
    match SearchService::search(&ctx, "typed", Some("gov")).unwrap() {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }

    // Replay should rebuild the search index correctly
    ctx.with_db(|conn| event::repo::replay(conn, PROJECTIONS).unwrap());
    match SearchService::search(&ctx, "architecture", None).unwrap() {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }
}

// ── System context tests ─────────────────────────────────────────

static SYSTEM_PROJECTIONS: &[&[Projection]] = &[
    TenantProjections.all(),
    ActorProjections.all(),
    BrainProjections.all(),
    TicketProjections.all(),
];

fn system_ctx() -> SystemContext {
    let conn = Connection::open_in_memory().expect("open db");
    migrations::migrate_system(&conn).expect("migrate");
    SystemContext::new(conn, SYSTEM_PROJECTIONS)
}

#[test]
fn tenant_create_and_list() {
    let ctx = system_ctx();

    match TenantService::create(&ctx, "acme".into()).unwrap() {
        TenantResponse::Created(t) => assert_eq!(t.name, TenantName::new("acme")),
        other => panic!("Expected Created, got {other:?}"),
    }

    match TenantService::list(&ctx).unwrap() {
        TenantResponse::Listed(tenants) => assert_eq!(tenants.len(), 1),
        other => panic!("Expected Listed, got {other:?}"),
    }
}

#[test]
fn actor_create_and_get() {
    let ctx = system_ctx();

    // Create a tenant first
    let tenant_id = match TenantService::create(&ctx, "acme".into()).unwrap() {
        TenantResponse::Created(t) => t.id.to_string(),
        other => panic!("Expected Created, got {other:?}"),
    };

    match ActorService::create(&ctx, tenant_id.clone(), "alice".into()).unwrap() {
        ActorResponse::Created(a) => {
            assert_eq!(a.name, ActorName::new("alice"));
            assert_eq!(a.tenant_id, tenant_id);
        }
        other => panic!("Expected Created, got {other:?}"),
    }

    match ActorService::list(&ctx).unwrap() {
        ActorResponse::Listed(actors) => assert_eq!(actors.len(), 1),
        other => panic!("Expected Listed, got {other:?}"),
    }
}

#[test]
fn brain_create_and_conflict() {
    let ctx = system_ctx();

    match BrainService::create(&ctx, "test-brain".into()).unwrap() {
        BrainResponse::Created(b) => assert_eq!(b.name, BrainName::new("test-brain")),
        other => panic!("Expected Created, got {other:?}"),
    }

    // Duplicate name should conflict
    assert!(matches!(
        BrainService::create(&ctx, "test-brain".into()),
        Err(BrainError::Conflict(_))
    ));

    match BrainService::get(&ctx, "test-brain").unwrap() {
        BrainResponse::Found(b) => assert_eq!(b.name, BrainName::new("test-brain")),
        other => panic!("Expected Found, got {other:?}"),
    }
}

#[test]
fn ticket_issue_and_validate() {
    let ctx = system_ctx();

    // Set up tenant + actor + brain
    let tenant_id = match TenantService::create(&ctx, "acme".into()).unwrap() {
        TenantResponse::Created(t) => t.id.to_string(),
        other => panic!("Expected Created, got {other:?}"),
    };
    let actor_id = match ActorService::create(&ctx, tenant_id, "alice".into()).unwrap() {
        ActorResponse::Created(a) => a.id,
        other => panic!("Expected Created, got {other:?}"),
    };
    match BrainService::create(&ctx, "test-brain".into()).unwrap() {
        BrainResponse::Created(_) => {}
        other => panic!("Expected Created, got {other:?}"),
    }

    // Issue a ticket
    let token = match TicketService::create(&ctx, actor_id, "test-brain".into()).unwrap() {
        TicketResponse::Created(t) => {
            assert_eq!(t.brain_name, "test-brain");
            t.token
        }
        other => panic!("Expected Created, got {other:?}"),
    };

    // Validate the token
    match TicketService::validate(&ctx, &token).unwrap() {
        TicketResponse::Validated(t) => assert_eq!(t.brain_name, "test-brain"),
        other => panic!("Expected Validated, got {other:?}"),
    }
}

// ── Storage IO test ──────────────────────────────────────────────

#[test]
fn storage_upload_and_retrieve_content() {
    let dir = tempfile::tempdir().unwrap();
    let ctx = project_ctx().with_config(crate::config::Config::new(dir.path()));

    let content = b"Hello, oneiros!";

    // Upload — returns the name, not the full entry; resolve ID via a subsequent get
    let stored_name = match StorageService::upload(
        &ctx,
        "test.txt".into(),
        "text/plain".into(),
        content.to_vec(),
    )
    .unwrap()
    {
        StorageResponse::StorageSet(name) => {
            assert_eq!(name, StorageName::new("test.txt"));
            name
        }
        other => panic!("Expected StorageSet, got {other:?}"),
    };

    // List to recover the entry and its ID
    let id_str = match StorageService::list(&ctx).unwrap() {
        StorageResponse::Entries(entries) => {
            assert_eq!(entries.len(), 1);
            let entry = &entries[0];
            assert_eq!(entry.name, stored_name);
            assert_eq!(entry.size, content.len() as u64);
            entry.id.to_string()
        }
        other => panic!("Expected Entries, got {other:?}"),
    };

    // Get metadata
    match StorageService::get(&ctx, &id_str).unwrap() {
        StorageResponse::StorageDetails(entry) => {
            assert_eq!(entry.name, StorageName::new("test.txt"))
        }
        other => panic!("Expected StorageDetails, got {other:?}"),
    }

    // Get content
    match StorageService::get_content(&ctx, &id_str).unwrap() {
        StorageResponse::Content(StorageContent { entry, data }) => {
            assert_eq!(entry.name, StorageName::new("test.txt"));
            assert_eq!(data, content);
        }
        other => panic!("Expected Content, got {other:?}"),
    }

    // Remove
    assert!(matches!(
        StorageService::remove(&ctx, &id_str).unwrap(),
        StorageResponse::StorageRemoved(_)
    ));

    // File should be gone
    assert!(!dir.path().join("blobs").join(&id_str).exists());

    // Metadata should be gone
    assert!(StorageService::get(&ctx, &id_str).is_err());
}
