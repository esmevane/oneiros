use oneiros_usage::*;

pub(crate) async fn creates_and_wakes_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend.exec("seed core").await?;

    let result = backend
        .exec("emerge newborn process --description 'A new agent' --output json")
        .await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("emerged"))),
        "expected emerged outcome in {outcomes:?}"
    );

    // Verify the agent exists
    let show_result = backend
        .exec("agent show newborn.process --output json")
        .await?;
    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    assert!(
        show_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("agent-details"))),
        "expected agent-details outcome after emerge"
    );

    Ok(())
}

pub(crate) async fn recede_retires_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend.exec("seed core").await?;

    backend
        .exec("agent create retiring process --description 'Will retire'")
        .await?;

    let result = backend
        .exec("recede retiring.process --output json")
        .await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("receded"))),
        "expected receded outcome in {outcomes:?}"
    );

    Ok(())
}
