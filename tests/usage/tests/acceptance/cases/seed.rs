use oneiros_usage::*;

pub(crate) async fn core_creates_default_levels<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let result = backend.exec("seed core --output json").await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("seed-complete"))),
        "expected seed-complete outcome in {outcomes:?}"
    );

    // Verify levels were created
    let list_result = backend.exec("level list --output json").await?;

    let list_outcomes = list_result.as_array().expect("expected array of outcomes");

    let levels_outcome = list_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("levels")))
        .expect("expected levels outcome");

    let levels = levels_outcome
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected levels data array");

    // Core seed includes working, session, project, archival, core
    assert!(
        levels.len() >= 4,
        "expected at least 4 levels from core seed, got {}",
        levels.len()
    );

    Ok(())
}
