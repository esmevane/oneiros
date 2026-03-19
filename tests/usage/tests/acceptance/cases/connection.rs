use oneiros_usage::*;

/// Helper: bootstrap with agent + nature + two cognitions to connect.
async fn setup_with_connectable_entities<B: Backend>(
    backend: &mut B,
) -> Result<(String, String), Box<dyn core::error::Error>> {
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
        .exec("nature set caused --description 'One thought produced another'")
        .await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;

    // Create two cognitions to connect
    let first = backend
        .exec("cognition add thinker.process observation 'First thought' --output json")
        .await?;
    let second = backend
        .exec("cognition add thinker.process observation 'Second thought' --output json")
        .await?;

    let first_ref = first
        .as_array()
        .and_then(|a| {
            a.iter()
                .find(|o| o.get("type") == Some(&serde_json::json!("cognition-added")))
        })
        .and_then(|o| o.get("data"))
        .and_then(|d| d.get("ref_token"))
        .and_then(|r| r.as_str())
        .expect("expected first cognition ref_token")
        .to_string();

    let second_ref = second
        .as_array()
        .and_then(|a| {
            a.iter()
                .find(|o| o.get("type") == Some(&serde_json::json!("cognition-added")))
        })
        .and_then(|o| o.get("data"))
        .and_then(|d| d.get("ref_token"))
        .and_then(|r| r.as_str())
        .expect("expected second cognition ref_token")
        .to_string();

    Ok((first_ref, second_ref))
}

pub(crate) async fn create<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    let (from_ref, to_ref) = setup_with_connectable_entities(&mut backend).await?;

    let cmd = format!("connection create caused {from_ref} {to_ref} --output json");
    let result = backend.exec(&cmd).await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("connection-created"))),
        "expected connection-created outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let result = backend.exec("connection list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-connections"))),
        "expected no-connections outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    let (from_ref, to_ref) = setup_with_connectable_entities(&mut backend).await?;

    let cmd = format!("connection create caused {from_ref} {to_ref}");
    backend.exec(&cmd).await?;

    let result = backend.exec("connection list --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    let connections = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("connections")))
        .expect("expected connections outcome");

    let data = connections
        .get("data")
        .and_then(|d| d.as_array())
        .expect("expected data array");

    assert_eq!(data.len(), 1);

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    let (from_ref, to_ref) = setup_with_connectable_entities(&mut backend).await?;

    let create_cmd = format!("connection create caused {from_ref} {to_ref} --output json");
    let create_result = backend.exec(&create_cmd).await?;

    let outcomes = create_result
        .as_array()
        .expect("expected array of outcomes");
    let created = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("connection-created")))
        .expect("expected connection-created outcome");

    let id = created
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|id| id.as_str())
        .expect("expected connection id");

    let show_cmd = format!("connection show {id} --output json");
    let show_result = backend.exec(&show_cmd).await?;
    let show_outcomes = show_result.as_array().expect("expected array of outcomes");

    let details = show_outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("connection-details")))
        .expect("expected connection-details outcome");

    let data = details.get("data").expect("expected data field");
    assert_eq!(data.get("nature").and_then(|n| n.as_str()), Some("caused"));

    Ok(())
}

pub(crate) async fn remove_by_id<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    let (from_ref, to_ref) = setup_with_connectable_entities(&mut backend).await?;

    let create_cmd = format!("connection create caused {from_ref} {to_ref} --output json");
    let create_result = backend.exec(&create_cmd).await?;

    let outcomes = create_result
        .as_array()
        .expect("expected array of outcomes");
    let created = outcomes
        .iter()
        .find(|o| o.get("type") == Some(&serde_json::json!("connection-created")))
        .expect("expected connection-created outcome");

    let id = created
        .get("data")
        .and_then(|d| d.get("id"))
        .and_then(|id| id.as_str())
        .expect("expected connection id");

    let remove_cmd = format!("connection remove {id} --output json");
    let remove_result = backend.exec(&remove_cmd).await?;
    let remove_outcomes = remove_result
        .as_array()
        .expect("expected array of outcomes");

    assert!(
        remove_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("connection-removed"))),
        "expected connection-removed outcome in {remove_outcomes:?}"
    );

    // Verify it's gone
    let list_result = backend.exec("connection list --output json").await?;
    let list_outcomes = list_result.as_array().expect("expected array of outcomes");

    assert!(
        list_outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("no-connections"))),
        "expected no-connections after removal in {list_outcomes:?}"
    );

    Ok(())
}
