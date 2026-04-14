mod acceptance;
mod dream_context;
mod harness;
mod workflows;

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn replay_reconstructs_read_models() {
    let app = TestApp::new()
        .await
        .unwrap()
        .init_system()
        .await
        .unwrap()
        .init_project()
        .await
        .unwrap();

    app.command("level set working --description 'Active'")
        .await
        .unwrap();
    app.command("level set session --description 'Session'")
        .await
        .unwrap();
    app.command("persona set test-persona --description 'A test persona' --prompt 'You are a test.'")
        .await
        .unwrap();
    app.command("agent create gov test-persona --description 'Governor' --prompt 'You govern'")
        .await
        .unwrap();
    app.command(r#"cognition add gov.test-persona observation "Test thought""#)
        .await
        .unwrap();

    let client = app.client();

    // Verify read models before replay
    let levels = client.level().list(&ListLevels::builder().build()).await.unwrap();
    match levels {
        LevelResponse::Levels(listed) => assert_eq!(listed.len(), 2),
        other => panic!("Expected Levels, got {other:?}"),
    }

    // Replay — resets all projections and re-applies all events
    app.command("project replay").await.unwrap();

    // Read models should be identical after replay
    let levels = client.level().list(&ListLevels::builder().build()).await.unwrap();
    match levels {
        LevelResponse::Levels(listed) => assert_eq!(listed.len(), 2),
        other => panic!("Expected Levels after replay, got {other:?}"),
    }

    let agent = client.agent().get(&AgentName::new("gov.test-persona")).await.unwrap();
    match agent {
        AgentResponse::AgentDetails(a) => {
            assert_eq!(a.data.name, AgentName::new("gov.test-persona"))
        }
        other => panic!("Expected AgentDetails after replay, got {other:?}"),
    }
}

#[tokio::test]
async fn storage_content_round_trips() {
    let app = TestApp::new()
        .await
        .unwrap()
        .init_system()
        .await
        .unwrap()
        .init_project()
        .await
        .unwrap();

    // Create a temp file to upload
    let file_path = app.base_url(); // just need the data dir
    let upload_dir = tempfile::tempdir().unwrap();
    let upload_path = upload_dir.path().join("test.txt");
    std::fs::write(&upload_path, b"Hello, oneiros!").unwrap();

    let rendered = app
        .command(&format!(
            "storage set test.txt {} --description 'A test file'",
            upload_path.display()
        ))
        .await
        .unwrap();

    let entry = match rendered.into_response() {
        Responses::Storage(StorageResponse::StorageSet(entry)) => {
            assert_eq!(entry.data.key.as_str(), "test.txt");
            entry
        }
        other => panic!("Expected StorageSet, got {other:?}"),
    };

    // Hash should be stable on re-read
    let client = app.client();
    match client
        .storage()
        .show(&GetStorage::builder().key("test.txt").build())
        .await
        .unwrap()
    {
        StorageResponse::StorageDetails(shown) => {
            assert_eq!(shown.data.hash, entry.data.hash);
        }
        other => panic!("Expected StorageDetails, got {other:?}"),
    }
}
