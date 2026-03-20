use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn set_and_show<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    // Create a temp file to upload
    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("test-doc.txt");
    std::fs::write(&file_path, "Hello, storage!")?;

    let set_cmd = format!(
        "storage set test-doc {} --description 'A test document'",
        file_path.display()
    );
    let set_response = backend.exec(&set_cmd).await?;

    assert!(
        matches!(set_response.data, Responses::Storage(StorageResponse::StorageSet(_))),
        "expected StorageSet, got {set_response:?}"
    );

    // Verify via show
    let show_response = backend.exec("storage show test-doc").await?;

    match show_response.data {
        Responses::Storage(StorageResponse::StorageDetails(entry)) => {
            assert_eq!(entry.name.as_str(), "test-doc");
        }
        other => panic!("expected StorageDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let response = backend.exec("storage list").await?;

    assert!(
        matches!(response.data, Responses::Storage(StorageResponse::NoEntries)),
        "expected NoEntries, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("doc.txt");
    std::fs::write(&file_path, "content")?;

    let cmd = format!("storage set my-doc {}", file_path.display());
    backend.exec(&cmd).await?;

    let response = backend.exec("storage list").await?;

    match response.data {
        Responses::Storage(StorageResponse::Entries(entries)) => {
            assert_eq!(entries.len(), 1);
        }
        other => panic!("expected Entries, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn remove<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("removable.txt");
    std::fs::write(&file_path, "temporary")?;

    let cmd = format!("storage set removable {}", file_path.display());
    backend.exec(&cmd).await?;

    let remove_response = backend.exec("storage remove removable").await?;

    assert!(
        matches!(remove_response.data, Responses::Storage(StorageResponse::StorageRemoved(_))),
        "expected StorageRemoved, got {remove_response:?}"
    );

    // Verify gone
    let list_response = backend.exec("storage list").await?;

    assert!(
        matches!(list_response.data, Responses::Storage(StorageResponse::NoEntries)),
        "expected NoEntries after removal, got {list_response:?}"
    );

    Ok(())
}
