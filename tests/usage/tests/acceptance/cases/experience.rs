use oneiros_usage::*;

/// Helper: bootstrap with persona + agent + sensation for experiences.
async fn setup_with_agent_and_sensation<B: Backend>(backend: &mut B) -> TestResult {
    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend
        .exec("persona set process --description 'Process agents'")
        .await?;
    backend
        .exec("sensation set caused --description 'One thought produced another'")
        .await?;
    backend
        .exec("agent create observer process --description 'An observing agent'")
        .await?;
    Ok(())
}

pub(crate) async fn create<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let result = backend
        .exec("experience create observer.process caused 'A caused B' --output json")
        .await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("experience-created"))),
        "expected experience-created outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let result = backend.exec("experience list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-experiences"))),
        "expected no-experiences outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    backend
        .exec("experience create observer.process caused 'First experience'")
        .await?;
    backend
        .exec("experience create observer.process caused 'Second experience'")
        .await?;

    let result = backend.exec("experience list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    let experiences = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("experiences")))
        .expect("expected experiences outcome");

    let data = experiences
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected data array");

    assert_eq!(data.len(), 2);

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let create_result = backend
        .exec("experience create observer.process caused 'Show me this' --output json")
        .await?;

    let outcomes = create_result
        .as_array()
        .expect("expected array of outcomes");
    let created = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("experience-created")))
        .expect("expected experience-created outcome");

    let id = created
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|id| id.as_str())
        .expect("expected id in experience-created data");

    let show_cmd = format!("experience show {id} --output json");
    let show_result = backend.exec(&show_cmd).await?;
    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let details = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("experience-details")))
        .expect("expected experience-details outcome");

    let data = details.get("data").expect("expected data field");
    assert_eq!(
        data.get("description").and_then(|d| d.as_str()),
        Some("Show me this")
    );

    Ok(())
}

pub(crate) async fn update_description<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let create_result = backend
        .exec("experience create observer.process caused 'Original' --output json")
        .await?;

    let outcomes = create_result
        .as_array()
        .expect("expected array of outcomes");
    let created = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("experience-created")))
        .expect("expected experience-created outcome");

    let id = created
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|id| id.as_str())
        .expect("expected id");

    let update_cmd =
        format!("experience update {id} --description 'Updated description' --output json");
    let update_result = backend.exec(&update_cmd).await?;
    let update_outcomes = update_result
        .as_array()
        .expect("expected array of outcomes");

    assert!(
        update_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("experience-updated"))),
        "expected experience-updated outcome in {update_outcomes:?}"
    );

    // Verify via show
    let show_cmd = format!("experience show {id} --output json");
    let show_result = backend.exec(&show_cmd).await?;
    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let details = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("experience-details")))
        .expect("expected experience-details outcome");

    let data = details.get("data").expect("expected data field");
    assert_eq!(
        data.get("description").and_then(|d| d.as_str()),
        Some("Updated description")
    );

    Ok(())
}
