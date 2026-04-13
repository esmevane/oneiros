use crate::*;
use crate::tests::acceptance::harness::*;

pub(crate) async fn init_creates_tenant_and_actor<B: Backend>() -> TestResult {
    let harness = Harness::<B>::started().await?;

    let response = harness.exec_json("system init --name test --yes").await?;

    assert!(
        matches!(
            response,
            Responses::System(SystemResponse::SystemInitialized(_))
        ),
        "expected SystemInitialized, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn init_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::started().await?;

    let prompt = harness.exec_prompt("system init --name test --yes").await?;

    assert!(!prompt.is_empty(), "system init prompt should not be empty");

    Ok(())
}

pub(crate) async fn init_is_idempotent<B: Backend>() -> TestResult {
    let harness = Harness::<B>::started().await?;

    harness.exec_json("system init --name test --yes").await?;

    let response = harness.exec_json("system init --name test --yes").await?;

    assert!(
        matches!(
            response,
            Responses::System(SystemResponse::HostAlreadyInitialized)
        ),
        "expected HostAlreadyInitialized, got {response:#?}"
    );

    Ok(())
}
