use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn init_creates_tenant_and_actor<B: Backend>() -> TestResult {
    let backend = B::start().await?;

    let response = backend.exec_json("system init --name test --yes").await?;

    assert!(
        matches!(
            response.data,
            Responses::System(SystemResponse::SystemInitialized(_))
        ),
        "expected SystemInitialized, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn init_prompt<B: Backend>() -> TestResult {
    let backend = B::start().await?;

    let prompt = backend.exec_prompt("system init --name test --yes").await?;

    assert!(!prompt.is_empty(), "system init prompt should not be empty");

    Ok(())
}

pub(crate) async fn init_is_idempotent<B: Backend>() -> TestResult {
    let backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;

    let response = backend.exec_json("system init --name test --yes").await?;

    assert!(
        matches!(
            response.data,
            Responses::System(SystemResponse::HostAlreadyInitialized)
        ),
        "expected HostAlreadyInitialized, got {response:#?}"
    );

    Ok(())
}
