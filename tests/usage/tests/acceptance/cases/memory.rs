use oneiros_usage::*;

/// Helper: bootstrap with persona + agent + level so memories have references.
async fn setup_with_agent_and_level<B: Backend>(backend: &mut B) -> TestResult {
    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend
        .exec("persona set process --description 'Process agents'")
        .await?;
    backend
        .exec("level set session --description 'Session context' --prompt 'For the session.'")
        .await?;
    backend
        .exec("agent create learner process --description 'A learning agent'")
        .await?;
    Ok(())
}

pub(crate) async fn add_creates_memory<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_level(&mut backend).await?;

    let result = backend
        .exec("memory add learner.process session 'A test memory' --output json")
        .await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("memory-added"))),
        "expected memory-added outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_level(&mut backend).await?;

    let result = backend.exec("memory list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-memories"))),
        "expected no-memories outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_level(&mut backend).await?;

    backend
        .exec("memory add learner.process session 'First memory'")
        .await?;
    backend
        .exec("memory add learner.process session 'Second memory'")
        .await?;

    let result = backend.exec("memory list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    let memories = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("memories")))
        .expect("expected memories outcome");

    let data = memories
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected data array");

    assert_eq!(data.len(), 2);

    Ok(())
}

pub(crate) async fn list_filters_by_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_level(&mut backend).await?;

    backend
        .exec("agent create other process --description 'Other agent'")
        .await?;

    backend
        .exec("memory add learner.process session 'Learner memory'")
        .await?;
    backend
        .exec("memory add other.process session 'Other memory'")
        .await?;

    let result = backend
        .exec("memory list --agent learner.process --output json")
        .await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    let memories = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("memories")))
        .expect("expected memories outcome");

    let data = memories
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected data array");

    assert_eq!(data.len(), 1);

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_level(&mut backend).await?;

    let add_result = backend
        .exec("memory add learner.process session 'Show me this' --output json")
        .await?;

    let outcomes = add_result.as_array().expect("expected array of outcomes");
    let added = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("memory-added")))
        .expect("expected memory-added outcome");

    let id = added
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|id| id.as_str())
        .expect("expected id in memory-added data");

    let show_cmd = format!("memory show {id} --output json");
    let show_result = backend.exec(&show_cmd).await?;
    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let details = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("memory-details")))
        .expect("expected memory-details outcome");

    let data = details.get("data").expect("expected data field");
    assert_eq!(
        data.get("content").and_then(|c| c.as_str()),
        Some("Show me this")
    );

    Ok(())
}
