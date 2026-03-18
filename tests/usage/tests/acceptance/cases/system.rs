use oneiros_usage::*;

pub(crate) async fn init_creates_tenant_and_actor<B: Backend>() -> TestResult {
    let backend = B::start().await?;

    let result = backend
        .exec("system init --name test --yes --output json")
        .await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("system-initialized"))),
        "expected system-initialized outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn init_is_idempotent<B: Backend>() -> TestResult {
    let backend = B::start().await?;

    backend
        .exec("system init --name test --yes --output json")
        .await?;

    let result = backend
        .exec("system init --name test --yes --output json")
        .await?;

    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("host-already-initialized"))),
        "expected host-already-initialized outcome in {outcomes:?}"
    );

    Ok(())
}
