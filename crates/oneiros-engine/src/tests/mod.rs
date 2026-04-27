mod acceptance;
mod dream_context;
mod harness;
mod workflows;

use crate::*;

// ── Internal mechanics tests ────────────────────────────────────
//
// These test infrastructure that isn't exposed through the CLI/HTTP
// surface: projection replay and storage content-addressing.
// Broadcast events are tested via SSE in the continuity workflow.
// Event serialization is covered implicitly by every workflow that
// persists and reads back data.

fn test_config(brain: &str) -> (Config, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("create tempdir");
    let config = Config::builder()
        .data_dir(dir.path().to_path_buf())
        .brain(BrainName::new(brain))
        .build();

    (config, dir)
}

async fn project_context() -> (ProjectContext, tempfile::TempDir) {
    let (config, dir) = test_config("test");
    let system = config.system();

    SystemService::init(&system, &InitSystem::builder().name("test").build())
        .await
        .unwrap();

    ProjectService::init(
        &system,
        &InitProject::builder().name(BrainName::new("test")).build(),
    )
    .await
    .unwrap();

    (config.project(), dir)
}

async fn seed_persona(context: &ProjectContext) {
    PersonaService::set(
        context,
        &SetPersona::builder()
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
        &CreateAgent::builder()
            .name("gov")
            .persona("test-persona")
            .description("Governor")
            .prompt("You govern")
            .build(),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn replay_reconstructs_read_models() {
    let (context, _dir) = project_context().await;

    LevelService::set(
        &context,
        &SetLevel::builder()
            .name("working")
            .description("Active")
            .prompt("")
            .build(),
    )
    .await
    .unwrap();
    LevelService::set(
        &context,
        &SetLevel::builder()
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
        &AddCognition::builder()
            .agent("gov.test-persona")
            .texture("observation")
            .content("Test thought")
            .build(),
    )
    .await
    .unwrap();

    // Verify read models before replay
    match LevelService::list(
        &context,
        &ListLevels {
            filters: SearchFilters::default(),
        },
    )
    .await
    .unwrap()
    {
        LevelResponse::Levels(levels) => assert_eq!(levels.len(), 2),
        other => panic!("Expected Listed, got {other:?}"),
    }

    // Replay — resets all projections and re-applies all events
    context.replay().unwrap();

    // Read models should be identical after replay
    match LevelService::list(
        &context,
        &ListLevels {
            filters: SearchFilters::default(),
        },
    )
    .await
    .unwrap()
    {
        LevelResponse::Levels(levels) => assert_eq!(levels.len(), 2),
        other => panic!("Expected Listed after replay, got {other:?}"),
    }
    match AgentService::get(
        &context,
        &GetAgent::builder()
            .key(AgentName::new("gov.test-persona"))
            .build(),
    )
    .await
    .unwrap()
    {
        AgentResponse::AgentDetails(a) => {
            assert_eq!(a.data.name(), &AgentName::new("gov.test-persona"))
        }
        other => panic!("Expected AgentDetails after replay, got {other:?}"),
    }
    match CognitionService::list(
        &context,
        &ListCognitions {
            agent: Some(AgentName::new("gov.test-persona")),
            texture: None,
            filters: SearchFilters::default(),
        },
    )
    .await
    .unwrap()
    {
        CognitionResponse::Cognitions(cogs) => assert_eq!(cogs.len(), 1),
        other => panic!("Expected Cognitions after replay, got {other:?}"),
    }
}

#[tokio::test]
async fn storage_content_round_trips() {
    let (context, _dir) = project_context().await;
    let content = b"Hello, oneiros!";

    let entry = match StorageService::upload(
        &context,
        &UploadStorage::builder()
            .key("test.txt")
            .description("A test file")
            .data(content.to_vec())
            .build(),
    )
    .await
    .unwrap()
    {
        StorageResponse::StorageSet(entry) => {
            assert_eq!(entry.data.key().as_str(), "test.txt");
            entry
        }
        other => panic!("Expected StorageSet, got {other:?}"),
    };

    // Get content — round-trips through compress/decompress
    let retrieved = StorageService::get_content(&context, &StorageKey::new("test.txt"))
        .await
        .unwrap();
    assert_eq!(retrieved, content);

    // Hash should be stable
    match StorageService::show(
        &context,
        &GetStorage::builder()
            .key(StorageKey::new("test.txt"))
            .build(),
    )
    .await
    .unwrap()
    {
        StorageResponse::StorageDetails(shown) => {
            assert_eq!(shown.data.hash(), entry.data.hash());
        }
        other => panic!("Expected StorageDetails, got {other:?}"),
    }
}
