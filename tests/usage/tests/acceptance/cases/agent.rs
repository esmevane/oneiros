use oneiros_usage::*;

/// Helper: bootstrap + seed a persona so agents can reference it.
async fn setup_with_persona<B: Backend>(backend: &mut B) -> TestResult {
    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend
        .exec("persona set process --description 'Process agents'")
        .await?;
    Ok(())
}

pub(crate) async fn create_with_persona<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    let result = backend
        .exec("agent create test process --description 'A test agent' --output json")
        .await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("agent-created"))),
        "expected agent-created outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn show_returns_details<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    backend
        .exec("agent create viewer process --description 'Views things'")
        .await?;

    let result = backend
        .exec("agent show viewer.process --output json")
        .await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    let agent = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("agent-details")))
        .expect("expected agent-details outcome");

    let data = agent.get("data").expect("expected data field");
    assert_eq!(
        data.get("name").and_then(|n| n.as_str()),
        Some("viewer.process")
    );
    assert_eq!(
        data.get("persona").and_then(|p| p.as_str()),
        Some("process")
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let result = backend.exec("agent list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-agents"))),
        "expected no-agents outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    backend
        .exec("agent create first process --description 'First'")
        .await?;
    backend
        .exec("agent create second process --description 'Second'")
        .await?;

    let result = backend.exec("agent list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    let agents_outcome = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("agents")))
        .expect("expected agents outcome");

    let agents = agents_outcome
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected agents data array");

    assert_eq!(agents.len(), 2);

    Ok(())
}

pub(crate) async fn update_changes_fields<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    backend
        .exec("agent create mutable process --description 'Original' --prompt 'Original.'")
        .await?;

    let result = backend
        .exec("agent update mutable.process process --description 'Updated' --prompt 'Updated.' --output json")
        .await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("agent-updated"))),
        "expected agent-updated outcome in {outcomes:?}"
    );

    // Verify via show
    let show_result = backend
        .exec("agent show mutable.process --output json")
        .await?;
    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let agent = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("agent-details")))
        .expect("expected agent-details outcome");

    let data = agent.get("data").expect("expected data field");
    assert_eq!(
        data.get("description").and_then(|d| d.as_str()),
        Some("Updated")
    );

    Ok(())
}

pub(crate) async fn remove_makes_it_unlisted<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    backend
        .exec("agent create temporary process --description 'Will be removed'")
        .await?;

    let result = backend
        .exec("agent remove temporary.process --output json")
        .await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("agent-removed"))),
        "expected agent-removed outcome in {outcomes:?}"
    );

    let list_result = backend.exec("agent list --output json").await?;
    let list_outcomes = list_result.as_array().expect("expected array of outcomes");

    assert!(
        list_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-agents"))),
        "expected no-agents after removal in {list_outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn name_includes_persona_suffix<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    // Create with bare name — should auto-append .process
    backend
        .exec("agent create governor process --description 'Governor'")
        .await?;

    let result = backend
        .exec("agent show governor.process --output json")
        .await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    let agent = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("agent-details")))
        .expect("expected agent-details outcome");

    let data = agent.get("data").expect("expected data field");
    assert_eq!(
        data.get("name").and_then(|n| n.as_str()),
        Some("governor.process")
    );

    Ok(())
}
