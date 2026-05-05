mod acceptance;
mod dream_context;
mod harness;
mod workflows;

use crate::tests::harness::TestApp;
use crate::*;

// ── Internal mechanics tests ────────────────────────────────────
//
// These test infrastructure that isn't directly exposed through HTTP:
// projection replay (driven via the `project replay` CLI command) and
// storage content-addressing (compress/decompress round-trip on the local
// blob, which has no HTTP fetch route by design).
//
// All bootstrap and write-side setup goes through TestApp + TestClient —
// the same path the binary takes. Pulling a ProjectLog out of the app is
// reserved for exercising the specific internal mechanism under test.

async fn booted_app() -> TestApp {
    TestApp::new()
        .await
        .expect("boot test app")
        .init_system()
        .await
        .expect("init system")
        .init_project()
        .await
        .expect("init project")
}

async fn seed_persona(client: &harness::TestClient) {
    client
        .persona()
        .set(
            &SetPersona::builder_v1()
                .name("test-persona")
                .description("A test persona")
                .prompt("You are a test.")
                .build()
                .into(),
        )
        .await
        .expect("seed persona");
}

async fn seed_agent(client: &harness::TestClient) {
    client
        .agent()
        .create(&CreateAgent::V1(
            CreateAgentV1::builder()
                .name("gov")
                .persona("test-persona")
                .description("Governor")
                .prompt("You govern")
                .build(),
        ))
        .await
        .expect("seed agent");
}

#[tokio::test]
async fn replay_reconstructs_read_models() {
    let app = booted_app().await;
    let client = app.client();

    client
        .level()
        .set(
            &SetLevel::builder_v1()
                .name("working")
                .description("Active")
                .prompt("")
                .build()
                .into(),
        )
        .await
        .expect("set working level");
    client
        .level()
        .set(
            &SetLevel::builder_v1()
                .name("session")
                .description("Session")
                .prompt("")
                .build()
                .into(),
        )
        .await
        .expect("set session level");
    seed_persona(&client).await;
    seed_agent(&client).await;
    client
        .cognition()
        .add(
            &AddCognition::builder_v1()
                .agent("gov.test-persona")
                .texture("observation")
                .content("Test thought")
                .build()
                .into(),
        )
        .await
        .expect("add cognition");

    // Verify read models before replay
    match client
        .level()
        .list(&ListLevels::builder_v1().build().into())
        .await
        .expect("list levels")
    {
        LevelResponse::Levels(LevelsResponse::V1(levels)) => assert_eq!(levels.items.len(), 2),
        other => panic!("Expected Listed, got {other:?}"),
    }

    // Replay through the CLI — same path the user takes.
    app.command("project replay")
        .await
        .expect("project replay command");

    // Read models should be identical after replay
    match client
        .level()
        .list(&ListLevels::builder_v1().build().into())
        .await
        .expect("list levels after replay")
    {
        LevelResponse::Levels(LevelsResponse::V1(levels)) => assert_eq!(levels.items.len(), 2),
        other => panic!("Expected Listed after replay, got {other:?}"),
    }
    match client
        .agent()
        .get(&GetAgent::V1(
            GetAgentV1::builder()
                .key(AgentName::new("gov.test-persona"))
                .build(),
        ))
        .await
        .expect("get agent after replay")
    {
        AgentResponse::AgentDetails(AgentDetailsResponse::V1(a)) => {
            assert_eq!(a.agent.name, AgentName::new("gov.test-persona"))
        }
        other => panic!("Expected AgentDetails after replay, got {other:?}"),
    }
    match client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(AgentName::new("gov.test-persona"))
                .build()
                .into(),
        )
        .await
        .expect("list cognitions after replay")
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
            assert_eq!(cogs.items.len(), 1)
        }
        other => panic!("Expected Cognitions after replay, got {other:?}"),
    }
}

#[tokio::test]
async fn replay_recovers_from_deleted_bookmark_db() {
    let app = booted_app().await;
    let client = app.client();

    seed_persona(&client).await;
    seed_agent(&client).await;
    client
        .cognition()
        .add(
            &AddCognition::builder_v1()
                .agent("gov.test-persona")
                .texture("observation")
                .content("Pre-nuke thought")
                .build()
                .into(),
        )
        .await
        .expect("add cognition");

    // Verify baseline before nuking the DB
    match client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(AgentName::new("gov.test-persona"))
                .build()
                .into(),
        )
        .await
        .expect("list cognitions baseline")
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
            assert_eq!(cogs.items.len(), 1);
        }
        other => panic!("Expected Cognitions before nuke, got {other:?}"),
    }

    // Simulate schema-change / corruption: delete the bookmark DB file
    let db_path = app.config().bookmark_db_path();
    std::fs::remove_file(&db_path).unwrap();
    let _ = std::fs::remove_file(db_path.with_extension("db-wal"));
    let _ = std::fs::remove_file(db_path.with_extension("db-shm"));

    // Replay through the CLI should recreate the DB and restore all data.
    app.command("project replay")
        .await
        .expect("project replay command");

    // Data should be fully restored
    match client
        .agent()
        .get(&GetAgent::V1(
            GetAgentV1::builder()
                .key(AgentName::new("gov.test-persona"))
                .build(),
        ))
        .await
        .expect("get agent after replay")
    {
        AgentResponse::AgentDetails(AgentDetailsResponse::V1(a)) => {
            assert_eq!(a.agent.name, AgentName::new("gov.test-persona"))
        }
        other => panic!("Expected AgentDetails after replay, got {other:?}"),
    }
    match client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(AgentName::new("gov.test-persona"))
                .build()
                .into(),
        )
        .await
        .expect("list cognitions after replay")
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
            assert_eq!(cogs.items.len(), 1);
        }
        other => panic!("Expected Cognitions after replay, got {other:?}"),
    }
}

#[tokio::test]
async fn storage_content_round_trips() {
    let app = booted_app().await;
    let client = app.client();
    let content = b"Hello, oneiros!";

    let entry = match client
        .storage()
        .upload(
            &UploadStorage::builder_v1()
                .key("test.txt")
                .description("A test file")
                .data(content.to_vec())
                .build()
                .into(),
        )
        .await
        .expect("upload storage")
    {
        StorageResponse::StorageSet(StorageSetResponse::V1(set)) => {
            assert_eq!(set.entry.key.as_str(), "test.txt");
            set.entry
        }
        other => panic!("Expected StorageSet, got {other:?}"),
    };

    // Hash should be stable — verify via the show client.
    match client
        .storage()
        .show(
            &GetStorage::builder_v1()
                .key(StorageKey::new("test.txt"))
                .build()
                .into(),
        )
        .await
        .expect("show storage")
    {
        StorageResponse::StorageDetails(StorageDetailsResponse::V1(shown)) => {
            assert_eq!(shown.entry.hash, entry.hash);
        }
        other => panic!("Expected StorageDetails, got {other:?}"),
    }

    // Internal mechanism under test: blob compression round-trip. There is
    // no HTTP route that returns raw blob bytes, so this single call
    // exercises the storage subsystem directly.
    let scope = ComposeScope::new(app.config().clone())
        .bookmark(app.config().brain.clone(), app.config().bookmark.clone())
        .expect("compose bookmark scope");
    let retrieved = StorageService::get_content(&scope, &StorageKey::new("test.txt"))
        .await
        .expect("get content");
    assert_eq!(retrieved, content);
}
