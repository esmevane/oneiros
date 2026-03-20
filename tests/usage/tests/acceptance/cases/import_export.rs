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

    let cmd = format!(
        "project export --target {}",
        export_dir.path().display()
    );
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
    let export_cmd = format!(
        "project export --target {}",
        export_dir.path().display()
    );
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
        matches!(show_response.data, Responses::Agent(AgentResponse::AgentDetails(_))),
        "expected agent to survive replay, got {show_response:?}"
    );

    Ok(())
}
