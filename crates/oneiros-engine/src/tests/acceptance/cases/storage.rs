use super::*;

pub(crate) async fn set_and_show<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    // Create a temp file to upload
    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("test-doc.txt");
    std::fs::write(&file_path, "Hello, storage!")?;

    let set_cmd = format!(
        "storage set test-doc {} --description 'A test document'",
        file_path.display()
    );
    let set_response = harness.exec_json(&set_cmd).await?;

    assert!(
        matches!(
            set_response,
            Responses::Storage(StorageResponse::StorageSet(_))
        ),
        "expected StorageSet, got {set_response:?}"
    );

    // Verify via show
    let show_response = harness.exec_json("storage show test-doc").await?;

    match show_response {
        Responses::Storage(StorageResponse::StorageDetails(entry)) => {
            assert_eq!(entry.data.key().as_str(), "test-doc");
        }
        other => panic!("expected StorageDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("storage list").await?;

    assert!(
        matches!(response, Responses::Storage(StorageResponse::NoEntries)),
        "expected NoEntries, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("doc.txt");
    std::fs::write(&file_path, "content")?;

    let cmd = format!("storage set my-doc {}", file_path.display());
    harness.exec_json(&cmd).await?;

    let response = harness.exec_json("storage list").await?;

    match response {
        Responses::Storage(StorageResponse::Entries(entries)) => {
            assert_eq!(entries.len(), 1);
        }
        other => panic!("expected Entries, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn set_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("doc.txt");
    std::fs::write(&file_path, "content")?;

    let prompt = harness
        .exec_prompt(&format!("storage set my-doc {}", file_path.display()))
        .await?;

    assert!(!prompt.is_empty(), "storage set prompt should not be empty");

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("doc.txt");
    std::fs::write(&file_path, "content")?;

    harness
        .exec_json(&format!("storage set my-doc {}", file_path.display()))
        .await?;

    let prompt = harness.exec_prompt("storage show my-doc").await?;

    assert!(
        !prompt.is_empty(),
        "storage show prompt should not be empty"
    );
    assert!(
        prompt.contains("my-doc"),
        "storage show prompt should contain the key"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("doc.txt");
    std::fs::write(&file_path, "content")?;

    harness
        .exec_json(&format!("storage set my-doc {}", file_path.display()))
        .await?;

    let prompt = harness.exec_prompt("storage list").await?;

    assert!(
        !prompt.is_empty(),
        "storage list prompt should not be empty when entries exist"
    );

    Ok(())
}

pub(crate) async fn remove_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("doc.txt");
    std::fs::write(&file_path, "content")?;

    harness
        .exec_json(&format!("storage set removable {}", file_path.display()))
        .await?;

    let prompt = harness.exec_prompt("storage remove removable").await?;

    assert!(
        !prompt.is_empty(),
        "storage remove prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn remove<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("removable.txt");
    std::fs::write(&file_path, "temporary")?;

    let cmd = format!("storage set removable {}", file_path.display());
    harness.exec_json(&cmd).await?;

    let remove_response = harness.exec_json("storage remove removable").await?;

    assert!(
        matches!(
            remove_response,
            Responses::Storage(StorageResponse::StorageRemoved(_))
        ),
        "expected StorageRemoved, got {remove_response:?}"
    );

    // Verify gone
    let list_response = harness.exec_json("storage list").await?;

    assert!(
        matches!(
            list_response,
            Responses::Storage(StorageResponse::NoEntries)
        ),
        "expected NoEntries after removal, got {list_response:?}"
    );

    Ok(())
}
