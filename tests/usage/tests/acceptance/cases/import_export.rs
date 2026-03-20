use oneiros_engine::*;
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

    let cmd = format!("project export --target {}", export_dir.path().display());
    let response = backend.exec(&cmd).await?;

    let export_path = match response.data {
        Responses::Project(ProjectResponse::WroteExport(path)) => path,
        other => panic!("expected WroteExport, got {other:#?}"),
    };

    // Verify file was created and is not empty
    assert!(export_path.exists(), "export file should exist");
    let content = std::fs::read_to_string(&export_path)?;
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
    let export_cmd = format!("project export --target {}", export_dir.path().display());
    let export_response = backend.exec(&export_cmd).await?;

    let export_path = match export_response.data {
        Responses::Project(ProjectResponse::WroteExport(path)) => path,
        other => panic!("expected WroteExport, got {other:#?}"),
    };

    // Import the exported file (idempotent — re-importing to same brain)
    backend
        .exec(&format!("project import {}", export_path.display()))
        .await?;

    // Verify data survived — the cognition should still be searchable
    let search_response = backend.exec("search Remember").await?;

    match search_response.data {
        Responses::Search(SearchResponse::Results(results)) => {
            assert!(
                !results.results.is_empty(),
                "expected to find the cognition after import"
            );
        }
        other => panic!("expected Search(Results), got {other:#?}"),
    }

    Ok(())
}

/// Storage entries with blob data should survive export from one brain and import
/// into a fresh brain — the distribution story in miniature.
///
/// Brain A produces events (including storage with binary blobs), exports them.
/// Brain B imports the JSONL and should have the same storage entries.
pub(crate) async fn export_import_preserves_storage<B: Backend>() -> TestResult {
    // Brain A — the source
    let mut brain_a = B::start().await?;
    brain_a.exec("system init --name test --yes").await?;
    brain_a.start_service().await?;
    brain_a.exec("project init --yes").await?;

    // Create a temp file and store it on brain A
    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("test.txt");
    std::fs::write(&file_path, "Portable blob content")?;

    let cmd = format!(
        "storage set portable-doc {} --description 'A portable document'",
        file_path.display()
    );
    brain_a.exec(&cmd).await?;

    // Export from brain A
    let export_dir = tempfile::TempDir::new()?;
    let export_cmd = format!("project export --target {}", export_dir.path().display());
    let export_response = brain_a.exec(&export_cmd).await?;

    let export_path = match export_response.data {
        Responses::Project(ProjectResponse::WroteExport(path)) => path,
        other => panic!("expected WroteExport, got {other:#?}"),
    };

    // Brain B — fresh, empty
    let mut brain_b = B::start().await?;
    brain_b.exec("system init --name test --yes").await?;
    brain_b.start_service().await?;
    brain_b.exec("project init --yes").await?;

    // Import brain A's export into brain B
    let import_cmd = format!("project import {}", export_path.display());
    brain_b.exec(&import_cmd).await?;

    // Brain B should now have the storage entry
    let show_response = brain_b.exec("storage show portable-doc").await?;

    match show_response.data {
        Responses::Storage(StorageResponse::StorageDetails(entry)) => {
            assert_eq!(entry.key.as_str(), "portable-doc");
        }
        other => panic!("expected StorageDetails on brain B after import, got {other:#?}"),
    }

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

    backend.exec("project replay").await?;

    // After replay, agent should still exist
    let show_response = backend.exec("agent show thinker.process").await?;

    assert!(
        matches!(
            show_response.data,
            Responses::Agent(AgentResponse::AgentDetails(_))
        ),
        "expected agent to survive replay, got {show_response:?}"
    );

    Ok(())
}
