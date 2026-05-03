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

async fn project_log() -> (ProjectLog, tempfile::TempDir) {
    let (config, dir) = test_config("test");
    let system = config.system();

    SystemService::init(
        &system,
        &InitSystem::builder_v1()
            .name("test".to_string())
            .build()
            .into(),
    )
    .await
    .unwrap();

    ProjectService::init(
        &system,
        &InitProject::builder_v1()
            .name(BrainName::new("test"))
            .build()
            .into(),
    )
    .await
    .unwrap();

    let mailboxes = Mailboxes::spawn(CanonIndex::new());
    (config.project().with_mailboxes(mailboxes), dir)
}

async fn seed_persona(context: &ProjectLog) {
    PersonaService::set(
        context,
        &SetPersona::builder_v1()
            .name("test-persona")
            .description("A test persona")
            .prompt("You are a test.")
            .build()
            .into(),
    )
    .await
    .unwrap();
}

async fn seed_agent(context: &ProjectLog) {
    AgentService::create(
        context,
        &CreateAgent::V1(
            CreateAgentV1::builder()
                .name("gov")
                .persona("test-persona")
                .description("Governor")
                .prompt("You govern")
                .build(),
        ),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn replay_reconstructs_read_models() {
    let (context, _dir) = project_log().await;

    LevelService::set(
        &context,
        &SetLevel::builder_v1()
            .name("working")
            .description("Active")
            .prompt("")
            .build()
            .into(),
    )
    .await
    .unwrap();
    LevelService::set(
        &context,
        &SetLevel::builder_v1()
            .name("session")
            .description("Session")
            .prompt("")
            .build()
            .into(),
    )
    .await
    .unwrap();
    seed_persona(&context).await;
    seed_agent(&context).await;
    CognitionService::add(
        &context,
        &AddCognition::builder_v1()
            .agent("gov.test-persona")
            .texture("observation")
            .content("Test thought")
            .build()
            .into(),
    )
    .await
    .unwrap();

    // Verify read models before replay
    match LevelService::list(&context, &ListLevels::builder_v1().build().into())
        .await
        .unwrap()
    {
        LevelResponse::Levels(LevelsResponse::V1(levels)) => assert_eq!(levels.items.len(), 2),
        other => panic!("Expected Listed, got {other:?}"),
    }

    // Replay — resets all projections and re-applies all events
    context.replay().unwrap();

    // Read models should be identical after replay
    match LevelService::list(&context, &ListLevels::builder_v1().build().into())
        .await
        .unwrap()
    {
        LevelResponse::Levels(LevelsResponse::V1(levels)) => assert_eq!(levels.items.len(), 2),
        other => panic!("Expected Listed after replay, got {other:?}"),
    }
    match AgentService::get(
        &context,
        &GetAgent::V1(
            GetAgentV1::builder()
                .key(AgentName::new("gov.test-persona"))
                .build(),
        ),
    )
    .await
    .unwrap()
    {
        AgentResponse::AgentDetails(AgentDetailsResponse::V1(a)) => {
            assert_eq!(a.agent.name, AgentName::new("gov.test-persona"))
        }
        other => panic!("Expected AgentDetails after replay, got {other:?}"),
    }
    match CognitionService::list(
        &context,
        &ListCognitions::builder_v1()
            .agent(AgentName::new("gov.test-persona"))
            .build()
            .into(),
    )
    .await
    .unwrap()
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
            assert_eq!(cogs.items.len(), 1)
        }
        other => panic!("Expected Cognitions after replay, got {other:?}"),
    }
}

#[tokio::test]
async fn replay_recovers_from_deleted_bookmark_db() {
    let (context, _dir) = project_log().await;

    seed_persona(&context).await;
    seed_agent(&context).await;
    CognitionService::add(
        &context,
        &AddCognition::builder_v1()
            .agent("gov.test-persona")
            .texture("observation")
            .content("Pre-nuke thought")
            .build()
            .into(),
    )
    .await
    .unwrap();

    // Verify baseline before nuking the DB — projections apply
    // eventually so the cognition may not be queryable immediately
    // after `CognitionService::add` returns.
    harness::Retryable::default()
        .wait_for_async(
            || async {
                match CognitionService::list(
                    &context,
                    &ListCognitions::builder_v1()
                        .agent(AgentName::new("gov.test-persona"))
                        .build()
                        .into(),
                )
                .await
                .map_err(|e| format!("{e:?}"))?
                {
                    CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
                        if cogs.items.len() == 1 {
                            Ok(())
                        } else {
                            Err(format!("expected 1, got {}", cogs.items.len()))
                        }
                    }
                    other => Err(format!("expected Cognitions, got {other:?}")),
                }
            },
            "cognition visible before nuke",
        )
        .await;

    // Simulate schema-change / corruption: delete the bookmark DB file
    let db_path = context.config.bookmark_db_path();
    std::fs::remove_file(&db_path).unwrap();
    let _ = std::fs::remove_file(db_path.with_extension("db-wal"));
    let _ = std::fs::remove_file(db_path.with_extension("db-shm"));

    // Replay should recreate the DB and restore all data
    match ProjectService::replay(&context).unwrap() {
        ProjectResponse::Replayed(ReplayedResponse::V1(result)) => {
            assert!(result.replayed > 0);
        }
        other => panic!("Expected Replayed, got {other:?}"),
    }

    // Data should be fully restored
    match AgentService::get(
        &context,
        &GetAgent::V1(
            GetAgentV1::builder()
                .key(AgentName::new("gov.test-persona"))
                .build(),
        ),
    )
    .await
    .unwrap()
    {
        AgentResponse::AgentDetails(AgentDetailsResponse::V1(a)) => {
            assert_eq!(a.agent.name, AgentName::new("gov.test-persona"))
        }
        other => panic!("Expected AgentDetails after replay, got {other:?}"),
    }
    match CognitionService::list(
        &context,
        &ListCognitions::builder_v1()
            .agent(AgentName::new("gov.test-persona"))
            .build()
            .into(),
    )
    .await
    .unwrap()
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
            assert_eq!(cogs.items.len(), 1);
        }
        other => panic!("Expected Cognitions after replay, got {other:?}"),
    }
}

#[tokio::test]
async fn storage_content_round_trips() {
    let (context, _dir) = project_log().await;
    let content = b"Hello, oneiros!";

    let entry = match StorageService::upload(
        &context,
        &UploadStorage::builder_v1()
            .key("test.txt")
            .description("A test file")
            .data(content.to_vec())
            .build()
            .into(),
    )
    .await
    .unwrap()
    {
        StorageResponse::StorageSet(StorageSetResponse::V1(set)) => {
            assert_eq!(set.entry.key.as_str(), "test.txt");
            set.entry
        }
        other => panic!("Expected StorageSet, got {other:?}"),
    };

    // Get content — round-trips through compress/decompress. Projections
    // apply asynchronously, so the storage entry may not be visible
    // immediately after upload returns.
    let retrieved = harness::Retryable::default()
        .wait_for_async(
            || async {
                StorageService::get_content(&context, &StorageKey::new("test.txt"))
                    .await
                    .map_err(|e| format!("{e:?}"))
            },
            "storage content available",
        )
        .await;
    assert_eq!(retrieved, content);

    // Hash should be stable
    match StorageService::show(
        &context,
        &GetStorage::builder_v1()
            .key(StorageKey::new("test.txt"))
            .build()
            .into(),
    )
    .await
    .unwrap()
    {
        StorageResponse::StorageDetails(StorageDetailsResponse::V1(shown)) => {
            assert_eq!(shown.entry.hash, entry.hash);
        }
        other => panic!("Expected StorageDetails, got {other:?}"),
    }
}
