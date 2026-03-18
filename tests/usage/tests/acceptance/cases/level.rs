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

pub(crate) async fn set_updates_existing_level<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    backend
        .exec("level set working --description 'Original description' --prompt 'Original prompt.'")
        .await?;

    backend
        .exec("level set working --description 'Updated description' --prompt 'Updated prompt.'")
        .await?;

    let show_result = backend.exec("level show working --output json").await?;

    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let level = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("level-details")))
        .expect("expected level-details outcome");

    let data = level.get("data").expect("expected data field");

    assert_eq!(
        data.get("description").and_then(|d| d.as_str()),
        Some("Updated description")
    );
    assert_eq!(
        data.get("prompt").and_then(|p| p.as_str()),
        Some("Updated prompt.")
    );

    Ok(())
}

pub(crate) async fn list_returns_empty_when_none_exist<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let result = backend.exec("level list --output json").await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-levels"))),
        "expected no-levels outcome in {outcomes:?}"
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

pub(crate) async fn remove_makes_it_unlisted<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    backend
        .exec("level set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let remove_result = backend.exec("level remove temporary --output json").await?;

    let remove_outcomes = remove_result
        .as_array()
        .expect("expected array of outcomes");

    assert!(
        remove_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("level-removed"))),
        "expected level-removed outcome in {remove_outcomes:?}"
    );

    // Verify it's gone
    let list_result = backend.exec("level list --output json").await?;

    let list_outcomes = list_result.as_array().expect("expected array of outcomes");

    assert!(
        list_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-levels"))),
        "expected no-levels after removal in {list_outcomes:?}"
    );

    Ok(())
}
