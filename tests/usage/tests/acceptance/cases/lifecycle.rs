use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: bootstrap with seeded vocabulary + an agent.
async fn setup_with_seeded_agent<B: Backend>(backend: &mut B) -> TestResult {
    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend.exec("seed core").await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;
    Ok(())
}

pub(crate) async fn wake<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec("wake thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Lifecycle(LifecycleResponse::Waking(_))
        ),
        "expected Waking, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn dream<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec("dream thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Lifecycle(LifecycleResponse::Dreaming(_))
        ),
        "expected Dreaming, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn introspect<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec("introspect thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Lifecycle(LifecycleResponse::Introspecting(_))
        ),
        "expected Introspecting, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn reflect<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec("reflect thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Lifecycle(LifecycleResponse::Reflecting(_))
        ),
        "expected Reflecting, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn sleep<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec("sleep thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Lifecycle(LifecycleResponse::Sleeping(_))
        ),
        "expected Sleeping, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn guidebook<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec("guidebook thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Lifecycle(LifecycleResponse::Guidebook(_))
        ),
        "expected Guidebook, got {response:#?}"
    );

    Ok(())
}
