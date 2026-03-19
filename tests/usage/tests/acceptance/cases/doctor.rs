use oneiros_usage::*;

pub(crate) async fn reports_initialized_system<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let result = backend.exec("doctor --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    // Should report system as initialized
    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("initialized"))),
        "expected initialized outcome in {outcomes:?}"
    );

    // Should find the database
    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("database-ok"))),
        "expected database-ok outcome in {outcomes:?}"
    );

    // Should have events in the log
    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("event-log-ready"))),
        "expected event-log-ready outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn reports_uninitialized_system<B: Backend>() -> TestResult {
    let backend = B::start().await?;

    let result = backend.exec("doctor --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    // Should report system as not initialized
    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("not-initialized"))),
        "expected not-initialized outcome in {outcomes:?}"
    );

    Ok(())
}
