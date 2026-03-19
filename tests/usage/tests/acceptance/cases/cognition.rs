use oneiros_usage::*;

/// Helper: bootstrap with persona + agent so cognitions have an agent to reference.
async fn setup_with_agent<B: Backend>(backend: &mut B) -> TestResult {
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
    Ok(())
}

pub(crate) async fn add_creates_cognition<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent(&mut backend).await?;

    let result = backend
        .exec("cognition add thinker.process observation 'A test thought' --output json")
        .await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("cognition-added"))),
        "expected cognition-added outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent(&mut backend).await?;

    let result = backend.exec("cognition list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-cognitions"))),
        "expected no-cognitions outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent(&mut backend).await?;

    backend
        .exec("cognition add thinker.process observation 'First thought'")
        .await?;
    backend
        .exec("cognition add thinker.process observation 'Second thought'")
        .await?;

    let result = backend.exec("cognition list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    let cognitions = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("cognitions")))
        .expect("expected cognitions outcome");

    let data = cognitions
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected data array");

    assert_eq!(data.len(), 2);

    Ok(())
}

pub(crate) async fn list_filters_by_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent(&mut backend).await?;

    backend
        .exec("agent create other process --description 'Other agent'")
        .await?;

    backend
        .exec("cognition add thinker.process observation 'Thinker thought'")
        .await?;
    backend
        .exec("cognition add other.process observation 'Other thought'")
        .await?;

    let result = backend
        .exec("cognition list --agent thinker.process --output json")
        .await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    let cognitions = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("cognitions")))
        .expect("expected cognitions outcome");

    let data = cognitions
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected data array");

    assert_eq!(data.len(), 1);

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent(&mut backend).await?;

    let add_result = backend
        .exec("cognition add thinker.process observation 'Show me this' --output json")
        .await?;

    let outcomes = add_result.as_array().expect("expected array of outcomes");
    let added = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("cognition-added")))
        .expect("expected cognition-added outcome");

    // Extract the ID from the added cognition
    let id = added
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|id| id.as_str())
        .expect("expected id in cognition-added data");

    let show_cmd = format!("cognition show {id} --output json");
    let show_result = backend.exec(&show_cmd).await?;
    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let details = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("cognition-details")))
        .expect("expected cognition-details outcome");

    let data = details.get("data").expect("expected data field");
    assert_eq!(
        data.get("content").and_then(|c| c.as_str()),
        Some("Show me this")
    );

    Ok(())
}
