use oneiros_usage::*;

pub(crate) async fn init_creates_brain<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend
        .exec("system init --name test --yes --output json")
        .await?;

    backend.start_service().await?;

    let result = backend.exec("project init --yes --output json").await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("brain-created"))),
        "expected brain-created outcome in {outcomes:?}"
    );

    Ok(())
}
