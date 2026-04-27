use super::*;

pub(crate) async fn export_produces_file<B: Backend>() -> TestResult {
    let harness = Harness::<B>::seed_project().await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;
    harness
        .exec_json("cognition add thinker.process observation 'An important thought'")
        .await?;

    // --target is a directory; the command constructs the filename
    let export_dir = tempfile::TempDir::new()?;

    let cmd = format!("project export --target {}", export_dir.path().display());
    let response = harness.exec_json(&cmd).await?;

    let export_path = match response {
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
    let harness = Harness::<B>::init_project().await?;
    harness
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    harness
        .exec_json("texture set observation --description 'Observations'")
        .await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;
    harness
        .exec_json("cognition add thinker.process observation 'Remember this thought'")
        .await?;

    // Export to a temp directory
    let export_dir = tempfile::TempDir::new()?;
    let export_cmd = format!("project export --target {}", export_dir.path().display());
    let export_response = harness.exec_json(&export_cmd).await?;

    let export_path = match export_response {
        Responses::Project(ProjectResponse::WroteExport(path)) => path,
        other => panic!("expected WroteExport, got {other:#?}"),
    };

    // Import the exported file (idempotent — re-importing to same brain)
    harness
        .exec_json(&format!("project import {}", export_path.display()))
        .await?;

    // Verify data survived — the cognition should still be searchable
    let search_response = harness.exec_json("search Remember").await?;

    match search_response {
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
    let brain_a = Harness::<B>::init_project().await?;

    // Create a temp file and store it on brain A
    let temp_dir = tempfile::TempDir::new()?;
    let file_path = temp_dir.path().join("test.txt");
    std::fs::write(&file_path, "Portable blob content")?;

    let cmd = format!(
        "storage set portable-doc {} --description 'A portable document'",
        file_path.display()
    );
    brain_a.exec_json(&cmd).await?;

    // Export from brain A
    let export_dir = tempfile::TempDir::new()?;
    let export_cmd = format!("project export --target {}", export_dir.path().display());
    let export_response = brain_a.exec_json(&export_cmd).await?;

    let export_path = match export_response {
        Responses::Project(ProjectResponse::WroteExport(path)) => path,
        other => panic!("expected WroteExport, got {other:#?}"),
    };

    // Brain B — fresh, empty
    let brain_b = Harness::<B>::init_project().await?;

    // Import brain A's export into brain B
    let import_cmd = format!("project import {}", export_path.display());
    brain_b.exec_json(&import_cmd).await?;

    // Brain B should now have the storage entry
    let show_response = brain_b.exec_json("storage show portable-doc").await?;

    match show_response {
        Responses::Storage(StorageResponse::StorageDetails(entry)) => {
            assert_eq!(entry.data.key().as_str(), "portable-doc");
        }
        other => panic!("expected StorageDetails on brain B after import, got {other:#?}"),
    }

    Ok(())
}

/// Import should be self-bootstrapping: a brain that has seen `system init`
/// but never `project init` should still accept an import and materialize
/// the data. This is the correctness-gate property the versioning design
/// leans on — "snapshot imported through new code produces same projection
/// state" presumes import can hydrate a fresh brain without relying on init
/// to have pre-migrated the on-disk schema.
pub(crate) async fn import_bootstraps_fresh_brain<B: Backend>() -> TestResult {
    let source = Harness::<B>::init_project().await?;
    source
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    source
        .exec_json("texture set observation --description 'Observations'")
        .await?;
    source
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;
    source
        .exec_json("cognition add thinker.process observation 'Remember this thought'")
        .await?;

    let export_dir = tempfile::TempDir::new()?;
    let export_cmd = format!("project export --target {}", export_dir.path().display());
    let export_response = source.exec_json(&export_cmd).await?;

    let export_path = match export_response {
        Responses::Project(ProjectResponse::WroteExport(path)) => path,
        other => panic!("expected WroteExport, got {other:#?}"),
    };

    // Destination has system init but no project init — import must
    // bootstrap the brain's schema itself.
    let destination = Harness::<B>::setup_system().await?.start_service().await?;

    let import_response = destination
        .exec_json(&format!("project import {}", export_path.display()))
        .await?;

    match import_response {
        Responses::Project(ProjectResponse::Imported(result)) => {
            assert!(
                result.imported.0 > 0,
                "expected at least one event imported, got {}",
                result.imported.0,
            );
            assert!(
                result.replayed.0 > 0,
                "expected at least one event replayed, got {}",
                result.replayed.0,
            );
        }
        other => panic!("expected Imported, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn replay_rebuilds_projections<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;
    harness
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    harness
        .exec_json("texture set observation --description 'Observations'")
        .await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    harness.exec_json("project replay").await?;

    // After replay, agent should still exist
    let show_response = harness.exec_json("agent show thinker.process").await?;

    assert!(
        matches!(
            show_response,
            Responses::Agent(AgentResponse::AgentDetails(_))
        ),
        "expected agent to survive replay, got {show_response:?}"
    );

    Ok(())
}
