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
        "storage set test-doc {} --description 'A test document' --output json",
        file_path.display()
    );
    let result = backend.exec(&set_cmd).await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("storage-set"))),
        "expected storage-set outcome in {outcomes:?}"
    );

    // Verify via show
    let show_result = backend.exec("storage show test-doc --output json").await?;
    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let details = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("storage-details")))
        .expect("expected storage-details outcome");

    let data = details.get("data").expect("expected data field");
    assert_eq!(data.get("key").and_then(|k| k.as_str()), Some("test-doc"));

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let result = backend.exec("storage list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-entries"))),
        "expected no-entries outcome in {outcomes:?}"
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

    let result = backend.exec("storage list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    let entries = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("entries")))
        .expect("expected entries outcome");

    let data = entries
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected data array");

    assert_eq!(data.len(), 1);

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

    let result = backend
        .exec("storage remove removable --output json")
        .await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("storage-removed"))),
        "expected storage-removed outcome in {outcomes:?}"
    );

    // Verify gone
    let list_result = backend.exec("storage list --output json").await?;
    let list_outcomes = list_result.as_array().expect("expected array of outcomes");

    assert!(
        list_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-entries"))),
        "expected no-entries after removal"
    );

    Ok(())
}
