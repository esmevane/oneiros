use oneiros_usage::*;

pub(crate) async fn list_shows_events<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend
        .exec("persona set process --description 'Process agents'")
        .await?;

    // There should be events from brain creation + persona set
    let result = backend.exec("event list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    // Should have at least one event (not empty)
    assert!(!outcomes.is_empty(), "expected at least one event outcome");

    Ok(())
}
