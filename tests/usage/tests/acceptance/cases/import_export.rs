use oneiros_usage::*;

pub(crate) async fn export_produces_file<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend.exec("seed core").await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;
    backend
        .exec("cognition add thinker.process observation 'An important thought'")
        .await?;

    // --target is a directory; the command constructs the filename
    let export_dir = tempfile::TempDir::new()?;

    let cmd = format!(
        "project export --target {} --output json",
        export_dir.path().display()
    );
    let result = backend.exec(&cmd).await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    // Extract the file path from the wrote-export outcome
    let wrote = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("wrote-export")))
        .expect("expected wrote-export outcome");

    let export_path_str = wrote
        .get("data")
        .and_then(|d| d.as_str())
        .expect("expected file path in wrote-export data");

    let export_path = std::path::Path::new(export_path_str);

    // Verify file was created and is not empty
    assert!(export_path.exists(), "export file should exist");
    let content = std::fs::read_to_string(export_path)?;
    assert!(!content.is_empty(), "export file should not be empty");

    // Count lines — should have at least a few events
    let line_count = content.lines().count();
    assert!(
        line_count >= 3,
        "expected at least 3 events in export, got {line_count}"
    );

    Ok(())
}

pub(crate) async fn import_restores_data<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend
        .exec("persona set process --description 'Process agents'")
        .await?;
    backend
        .exec("texture set observation --description 'Observations'")
        .await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;
    backend
        .exec("cognition add thinker.process observation 'Remember this thought'")
        .await?;

    // Export to a temp directory
    let export_dir = tempfile::TempDir::new()?;
    let export_cmd = format!(
        "project export --target {} --output json",
        export_dir.path().display()
    );
    let export_result = backend.exec(&export_cmd).await?;

    // Get the actual export file path
    let export_path = export_result
        .as_array()
        .and_then(|a| {
            a.iter()
                .find(|o| o.get("type") == Some(&serde_json::json!("wrote-export")))
        })
        .and_then(|o| o.get("data"))
        .and_then(|d| d.as_str())
        .expect("expected export file path");

    // Import the exported file (idempotent — re-importing to same brain)
    let import_cmd = format!("project import {} --output json", export_path);
    let result = backend.exec(&import_cmd).await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("imported"))),
        "expected imported outcome in {outcomes:?}"
    );

    // Verify data survived — the cognition should still be searchable
    let search_result = backend.exec("search Remember --output json").await?;
    let search_outcomes = search_result.as_array().expect("expected array");

    let results = search_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("results")))
        .and_then(|o| o.get("data"))
        .and_then(|d| d.get("results"))
        .and_then(|r| r.as_array())
        .expect("expected search results");

    assert!(
        results.len() >= 1,
        "expected to find the cognition after import"
    );

    Ok(())
}

pub(crate) async fn replay_rebuilds_projections<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend
        .exec("persona set process --description 'Process agents'")
        .await?;
    backend
        .exec("texture set observation --description 'Observations'")
        .await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;

    let result = backend.exec("project replay --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("replayed"))),
        "expected replayed outcome in {outcomes:?}"
    );

    // After replay, agent should still exist
    let show_result = backend
        .exec("agent show thinker.process --output json")
        .await?;
    let show_outcomes = show_result.as_array().expect("expected array");

    assert!(
        show_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("agent-details"))),
        "expected agent to survive replay"
    );

    Ok(())
}
