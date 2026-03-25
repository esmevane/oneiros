mod dream_context;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use crate::*;

// ── Helpers ───────────────────────────────────────────────────────

/// Create a bootstrapped Config in a tempdir.
///
/// Returns the config and the tempdir handle (which must be held
/// to keep the directory alive for the test's duration).
fn test_config(brain: &str) -> (Config, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("create tempdir");
    let config = Config::builder()
        .data_dir(dir.path().to_path_buf())
        .brain(BrainName::new(brain))
        .build();

    config.bootstrap().expect("bootstrap");

    (config, dir)
}

async fn project_context() -> (ProjectContext, tempfile::TempDir) {
    let (config, dir) = test_config("test");
    let system = config.system();

    // Create system entities so ProjectService::init can issue a ticket.
    SystemService::init(&system, "test".to_string())
        .await
        .unwrap();

    // Create brain + ticket.
    ProjectService::init(&system, BrainName::new("test"))
        .await
        .unwrap();

    (config.project(), dir)
}

async fn seed_persona(context: &ProjectContext) {
    PersonaService::set(
        context,
        Persona::builder()
            .name("test-persona")
            .description("A test persona")
            .prompt("You are a test.")
            .build(),
    )
    .await
    .unwrap();
}

async fn seed_agent(context: &ProjectContext) {
    AgentService::create(
        context,
        "gov".into(),
        "test-persona".into(),
        "Governor".into(),
        "You govern.".into(),
    )
    .await
    .unwrap();
}

// ── Vocabulary domain tests ───────────────────────────────────────

#[tokio::test]
async fn level_crud() {
    let (context, _dir) = project_context().await;

    LevelService::set(
        &context,
        Level::builder()
            .name("working")
            .description("Active")
            .prompt("")
            .build(),
    )
    .await
    .unwrap();
    assert!(matches!(
        LevelService::get(&context, &LevelName::new("working"))
            .await
            .unwrap(),
        LevelResponse::LevelDetails(_)
    ));

    match LevelService::list(&context).await.unwrap() {
        LevelResponse::Levels(levels) => assert_eq!(levels.len(), 1),
        other => panic!("Expected Listed, got {other:?}"),
    }

    LevelService::remove(&context, &LevelName::new("working"))
        .await
        .unwrap();
    assert!(
        LevelService::get(&context, &LevelName::new("working"))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn persona_crud() {
    let (context, _dir) = project_context().await;

    PersonaService::set(
        &context,
        Persona::builder()
            .name("process")
            .description("Process agents")
            .prompt("")
            .build(),
    )
    .await
    .unwrap();
    assert!(matches!(
        PersonaService::get(&context, &PersonaName::new("process"))
            .await
            .unwrap(),
        PersonaResponse::PersonaDetails(_)
    ));
}

// ── Entity domain tests ──────────────────────────────────────────

#[tokio::test]
async fn agent_create_and_get() {
    let (context, _dir) = project_context().await;
    seed_persona(&context).await;

    let resp = AgentService::create(
        &context,
        AgentName::new("governor"),
        PersonaName::new("test-persona"),
        Description::new("The governor"),
        Prompt::new("You govern."),
    )
    .await
    .unwrap();
    assert!(matches!(resp, AgentResponse::AgentCreated(_)));

    match AgentService::get(&context, &AgentName::new("governor.test-persona"))
        .await
        .unwrap()
    {
        AgentResponse::AgentDetails(a) => {
            assert_eq!(a.name, AgentName::new("governor.test-persona"));
            assert_eq!(a.persona, PersonaName::new("test-persona"));
        }
        other => panic!("Expected AgentDetails, got {other:?}"),
    }
}

#[tokio::test]
async fn agent_persona_validation() {
    let (context, _dir) = project_context().await;

    let result = AgentService::create(
        &context,
        AgentName::new("gov"),
        PersonaName::new("nonexistent"),
        Description::new(""),
        Prompt::new(""),
    )
    .await;
    assert!(matches!(result, Err(AgentError::PersonaNotFound(_))));
}

#[tokio::test]
async fn agent_name_conflict() {
    let (context, _dir) = project_context().await;
    seed_persona(&context).await;

    AgentService::create(
        &context,
        AgentName::new("gov"),
        PersonaName::new("test-persona"),
        Description::new(""),
        Prompt::new(""),
    )
    .await
    .unwrap();
    let result = AgentService::create(
        &context,
        AgentName::new("gov"),
        PersonaName::new("test-persona"),
        Description::new(""),
        Prompt::new(""),
    )
    .await;
    assert!(matches!(result, Err(AgentError::Conflict(_))));
}

#[tokio::test]
async fn cognition_add_and_list() {
    let (context, _dir) = project_context().await;
    seed_persona(&context).await;
    seed_agent(&context).await;

    let resp = CognitionService::add(
        &context,
        AgentName::new("gov.test-persona"),
        TextureName::new("observation"),
        Content::new("Something interesting"),
    )
    .await
    .unwrap();
    assert!(matches!(resp, CognitionResponse::CognitionAdded(_)));

    match CognitionService::list(&context, Some(AgentName::new("gov.test-persona")), None)
        .await
        .unwrap()
    {
        CognitionResponse::Cognitions(cogs) => assert_eq!(cogs.len(), 1),
        other => panic!("Expected Cognitions, got {other:?}"),
    }
}

// ── Typed event tests ────────────────────────────────────────────

#[tokio::test]
async fn broadcast_events_are_typed() {
    let (context, _dir) = project_context().await;
    seed_persona(&context).await;

    let mut sub = context.subscribe();

    LevelService::set(
        &context,
        Level::builder()
            .name("working")
            .description("")
            .prompt("")
            .build(),
    )
    .await
    .unwrap();
    let event = sub.try_recv().unwrap();
    assert!(matches!(
        event.data,
        Events::Level(LevelEvents::LevelSet(_))
    ));

    AgentService::create(
        &context,
        AgentName::new("gov"),
        PersonaName::new("test-persona"),
        Description::new(""),
        Prompt::new(""),
    )
    .await
    .unwrap();
    let event = sub.try_recv().unwrap();
    assert!(matches!(
        event.data,
        Events::Agent(AgentEvents::AgentCreated(_))
    ));
}

// ── Replay test ──────────────────────────────────────────────────

#[tokio::test]
async fn replay_reconstructs_read_models() {
    let (context, _dir) = project_context().await;

    // Seed data across multiple domains
    LevelService::set(
        &context,
        Level::builder()
            .name("working")
            .description("Active")
            .prompt("")
            .build(),
    )
    .await
    .unwrap();
    LevelService::set(
        &context,
        Level::builder()
            .name("session")
            .description("Session")
            .prompt("")
            .build(),
    )
    .await
    .unwrap();
    seed_persona(&context).await;
    seed_agent(&context).await;
    CognitionService::add(
        &context,
        AgentName::new("gov.test-persona"),
        TextureName::new("observation"),
        Content::new("Test thought"),
    )
    .await
    .unwrap();

    // Verify read models before replay
    match LevelService::list(&context).await.unwrap() {
        LevelResponse::Levels(levels) => assert_eq!(levels.len(), 2),
        other => panic!("Expected Listed, got {other:?}"),
    }
    assert!(matches!(
        AgentService::get(&context, &AgentName::new("gov.test-persona"))
            .await
            .unwrap(),
        AgentResponse::AgentDetails(_)
    ));

    // Replay — this resets all projections and re-applies all events
    context.replay().unwrap();

    // Read models should be identical after replay
    match LevelService::list(&context).await.unwrap() {
        LevelResponse::Levels(levels) => assert_eq!(levels.len(), 2),
        other => panic!("Expected Listed after replay, got {other:?}"),
    }
    match AgentService::get(&context, &AgentName::new("gov.test-persona"))
        .await
        .unwrap()
    {
        AgentResponse::AgentDetails(a) => assert_eq!(a.name, AgentName::new("gov.test-persona")),
        other => panic!("Expected AgentDetails after replay, got {other:?}"),
    }
    match CognitionService::list(&context, Some(AgentName::new("gov.test-persona")), None)
        .await
        .unwrap()
    {
        CognitionResponse::Cognitions(cogs) => assert_eq!(cogs.len(), 1),
        other => panic!("Expected Cognitions after replay, got {other:?}"),
    }
}

// ── HTTP collector tests ──────────────────────────────────────────

/// Create a test config with system + project + ticket, returning
/// the unified router, the Bearer token, and the tempdir handle.
async fn http_setup() -> (axum::Router, String, tempfile::TempDir) {
    let (config, dir) = test_config("test-brain");
    let system = config.system();

    // Create system entities.
    SystemService::init(&system, "test".to_string())
        .await
        .unwrap();

    // Create brain + ticket via ProjectService.
    let token = match ProjectService::init(&system, BrainName::new("test-brain"))
        .await
        .unwrap()
    {
        ProjectResponse::Initialized(result) => result.token,
        other => panic!("expected Initialized, got {other:?}"),
    };

    // Seed a persona into the project context for downstream tests
    let project = config.project();
    seed_persona(&project).await;

    let server = Server::new(config);
    (server.router(), token.to_string(), dir)
}

fn authed_get(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .header("Authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

fn authed_post(uri: &str, token: &str, body: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .body(Body::from(body.to_string()))
        .unwrap()
}

fn authed_put(uri: &str, token: &str, body: &str) -> Request<Body> {
    Request::builder()
        .method("PUT")
        .uri(uri)
        .header("content-type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::test]
async fn http_serves_multiple_domains() {
    let (app, token, _dir) = http_setup().await;

    // Set a level
    let resp = app
        .clone()
        .oneshot(authed_put(
            "/levels/working",
            &token,
            r#"{"name":"working","description":"Active","prompt":""}"#,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Create an agent
    let resp = app
        .clone()
        .oneshot(authed_post(
            "/agents",
            &token,
            r#"{"name":"gov","persona":"test-persona","description":"Gov","prompt":""}"#,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Read both back
    let resp = app
        .clone()
        .oneshot(authed_get("/levels/working", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = app
        .clone()
        .oneshot(authed_get("/agents/gov.test-persona", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn http_not_found() {
    let (app, token, _dir) = http_setup().await;

    let resp = app
        .clone()
        .oneshot(authed_get("/agents/nope", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let resp = app
        .oneshot(authed_get("/levels/nope", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ── Full integration ──────────────────────────────────────────────

#[tokio::test]
async fn full_integration() {
    let (app, token, _dir) = http_setup().await;

    // Create agent via HTTP
    let resp = app
        .clone()
        .oneshot(authed_post(
            "/agents",
            &token,
            r#"{"name":"gov","persona":"test-persona","description":"Gov","prompt":""}"#,
        ))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Read back via HTTP
    let resp = app
        .clone()
        .oneshot(authed_get("/agents/gov.test-persona", &token))
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// ── Event serialization round-trip ───────────────────────────────

#[test]
fn events_serialize_and_deserialize() {
    let event = Events::Level(LevelEvents::LevelSet(
        Level::builder()
            .name("test")
            .description("desc")
            .prompt("p")
            .build(),
    ));

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

#[tokio::test]
async fn search_indexes_across_domains() {
    let (context, _dir) = project_context().await;
    seed_persona(&context).await;
    seed_agent(&context).await;

    // Add cognitions
    CognitionService::add(
        &context,
        AgentName::new("gov.test-persona"),
        TextureName::new("observation"),
        Content::new("The architecture is clean"),
    )
    .await
    .unwrap();
    CognitionService::add(
        &context,
        AgentName::new("gov.test-persona"),
        TextureName::new("working"),
        Content::new("Working on typed events"),
    )
    .await
    .unwrap();

    // Search should find them
    match SearchService::search(&context, "architecture", None)
        .await
        .unwrap()
    {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }

    // Agent itself should be indexed too (from seed_agent)
    match SearchService::search(&context, "Governor", None)
        .await
        .unwrap()
    {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }

    // Search with agent filter
    match SearchService::search(&context, "typed", Some(&AgentName::new("gov.test-persona")))
        .await
        .unwrap()
    {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }

    // Replay should rebuild the search index correctly
    context.replay().unwrap();
    match SearchService::search(&context, "architecture", None)
        .await
        .unwrap()
    {
        SearchResponse::Results(r) => assert_eq!(r.results.len(), 1),
    }
}

// ── System context tests ─────────────────────────────────────────

fn system_context() -> (SystemContext, tempfile::TempDir) {
    let (config, dir) = test_config("test");
    (config.system(), dir)
}

#[tokio::test]
async fn tenant_create_and_list() {
    let (context, _dir) = system_context();

    match TenantService::create(&context, "acme".into())
        .await
        .unwrap()
    {
        TenantResponse::Created(t) => assert_eq!(t.name, TenantName::new("acme")),
        other => panic!("Expected Created, got {other:?}"),
    }

    match TenantService::list(&context).await.unwrap() {
        TenantResponse::Listed(tenants) => assert_eq!(tenants.len(), 1),
        other => panic!("Expected Listed, got {other:?}"),
    }
}

#[tokio::test]
async fn actor_create_and_get() {
    let (context, _dir) = system_context();

    // Create a tenant first
    let tenant = match TenantService::create(&context, "acme".into())
        .await
        .unwrap()
    {
        TenantResponse::Created(t) => t,
        other => panic!("Expected Created, got {other:?}"),
    };
    let tenant_id_str = tenant.id.to_string();

    match ActorService::create(&context, tenant.id, ActorName::new("alice"))
        .await
        .unwrap()
    {
        ActorResponse::Created(a) => {
            assert_eq!(a.name, ActorName::new("alice"));
            assert_eq!(a.tenant_id, tenant_id_str.parse::<TenantId>().unwrap());
        }
        other => panic!("Expected Created, got {other:?}"),
    }

    match ActorService::list(&context).await.unwrap() {
        ActorResponse::Listed(actors) => assert_eq!(actors.len(), 1),
        other => panic!("Expected Listed, got {other:?}"),
    }
}

#[tokio::test]
async fn brain_create_and_conflict() {
    let (context, _dir) = system_context();

    match BrainService::create(&context, "test-brain".into())
        .await
        .unwrap()
    {
        BrainResponse::Created(b) => assert_eq!(b.name, BrainName::new("test-brain")),
        other => panic!("Expected Created, got {other:?}"),
    }

    // Duplicate name should conflict
    assert!(matches!(
        BrainService::create(&context, "test-brain".into()).await,
        Err(BrainError::Conflict(_))
    ));

    match BrainService::get(&context, &BrainName::new("test-brain"))
        .await
        .unwrap()
    {
        BrainResponse::Found(b) => assert_eq!(b.name, BrainName::new("test-brain")),
        other => panic!("Expected Found, got {other:?}"),
    }
}

#[tokio::test]
async fn ticket_issue_and_validate() {
    let (context, _dir) = system_context();

    // Set up tenant + actor + brain
    let tenant_id = match TenantService::create(&context, "acme".into())
        .await
        .unwrap()
    {
        TenantResponse::Created(t) => t.id,
        other => panic!("Expected Created, got {other:?}"),
    };
    let actor_id = match ActorService::create(&context, tenant_id, ActorName::new("alice"))
        .await
        .unwrap()
    {
        ActorResponse::Created(a) => a.id,
        other => panic!("Expected Created, got {other:?}"),
    };
    match BrainService::create(&context, "test-brain".into())
        .await
        .unwrap()
    {
        BrainResponse::Created(_) => {}
        other => panic!("Expected Created, got {other:?}"),
    }

    // Issue a ticket
    let token = match TicketService::create(&context, actor_id, "test-brain".into())
        .await
        .unwrap()
    {
        TicketResponse::Created(t) => {
            assert_eq!(t.brain_name, BrainName::new("test-brain"));
            t.token
        }
        other => panic!("Expected Created, got {other:?}"),
    };

    // Validate the token
    match TicketService::validate(&context, token.as_str())
        .await
        .unwrap()
    {
        TicketResponse::Validated(t) => assert_eq!(t.brain_name, BrainName::new("test-brain")),
        other => panic!("Expected Validated, got {other:?}"),
    }
}

// ── Storage tests (content-addressed model) ─────────────────────

#[tokio::test]
async fn storage_upload_and_retrieve_content() {
    let (context, _dir) = project_context().await;
    let content = b"Hello, oneiros!";

    // Upload — returns the entry with key, description, hash
    let entry = match StorageService::upload(
        &context,
        StorageKey::new("test.txt"),
        Description::new("A test file"),
        content.to_vec(),
    )
    .await
    .unwrap()
    {
        StorageResponse::StorageSet(entry) => {
            assert_eq!(entry.key.as_str(), "test.txt");
            assert_eq!(entry.description.as_str(), "A test file");
            entry
        }
        other => panic!("Expected StorageSet, got {other:?}"),
    };

    // Show by key
    match StorageService::show(&context, &StorageKey::new("test.txt"))
        .await
        .unwrap()
    {
        StorageResponse::StorageDetails(shown) => {
            assert_eq!(shown.key.as_str(), "test.txt");
            assert_eq!(shown.hash, entry.hash);
        }
        other => panic!("Expected StorageDetails, got {other:?}"),
    }

    // List
    match StorageService::list(&context).await.unwrap() {
        StorageResponse::Entries(entries) => {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].key.as_str(), "test.txt");
        }
        other => panic!("Expected Entries, got {other:?}"),
    }

    // Get content — round-trips through compress/decompress
    let retrieved = StorageService::get_content(&context, &StorageKey::new("test.txt"))
        .await
        .unwrap();
    assert_eq!(retrieved, content);

    // Remove — only removes metadata, blob stays (dedup)
    assert!(matches!(
        StorageService::remove(&context, &StorageKey::new("test.txt"))
            .await
            .unwrap(),
        StorageResponse::StorageRemoved(_)
    ));

    // Metadata should be gone
    assert!(
        StorageService::show(&context, &StorageKey::new("test.txt"))
            .await
            .is_err()
    );

    // List should be empty
    assert!(matches!(
        StorageService::list(&context).await.unwrap(),
        StorageResponse::NoEntries
    ));
}
