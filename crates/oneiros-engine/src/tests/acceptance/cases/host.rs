use super::*;

pub(crate) async fn init_creates_tenant_and_actor<B: Backend>() -> TestResult {
    let harness = Harness::<B>::started().await?;

    let response = harness.exec_json("host init --name test --yes").await?;

    assert!(
        matches!(response, Responses::Host(HostResponse::HostInitialized(_))),
        "expected HostInitialized, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn init_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::started().await?;

    let prompt = harness.exec_prompt("host init --name test --yes").await?;

    assert!(!prompt.is_empty(), "host init prompt should not be empty");

    Ok(())
}

pub(crate) async fn init_is_idempotent<B: Backend>() -> TestResult {
    let harness = Harness::<B>::started().await?;

    harness.exec_json("host init --name test --yes").await?;

    let response = harness.exec_json("host init --name test --yes").await?;

    assert!(
        matches!(
            response,
            Responses::Host(HostResponse::HostAlreadyInitialized)
        ),
        "expected HostAlreadyInitialized, got {response:#?}"
    );

    Ok(())
}
