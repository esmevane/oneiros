use oneiros_usage::*;

pub(crate) async fn set_creates_a_new_level<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let result = backend
        .exec("level set ephemeral --description 'Short-lived context' --prompt 'Use for thoughts that will not outlast the session.' --output json")
        .await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("level-set"))),
        "expected level-set outcome in {outcomes:?}"
    );

    // Verify the level exists via show command
    let show_result = backend.exec("level show ephemeral --output json").await?;

    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let level = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("level-details")))
        .expect("expected level-details outcome");

    let data = level.get("data").expect("expected data field");

    assert_eq!(data.get("name").and_then(|n| n.as_str()), Some("ephemeral"));
    assert_eq!(
        data.get("description").and_then(|d| d.as_str()),
        Some("Short-lived context")
    );

    Ok(())
}

pub(crate) async fn list_returns_created_levels<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    backend
        .exec("level set session --description 'Session context' --prompt 'For the session.'")
        .await?;

    backend
        .exec("level set project --description 'Project knowledge' --prompt 'For the project.'")
        .await?;

    let result = backend.exec("level list --output json").await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    let levels_outcome = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("levels")))
        .expect("expected levels outcome");

    let levels = levels_outcome
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected levels data array");

    assert_eq!(levels.len(), 2);

    Ok(())
}
