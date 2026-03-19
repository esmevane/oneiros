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

    let result = backend.exec("wake thinker.process --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("waking"))),
        "expected waking outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn dream<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let result = backend.exec("dream thinker.process --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("dreaming"))),
        "expected dreaming outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn introspect<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let result = backend
        .exec("introspect thinker.process --output json")
        .await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("introspecting"))),
        "expected introspecting outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn reflect<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let result = backend
        .exec("reflect thinker.process --output json")
        .await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("reflecting"))),
        "expected reflecting outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn sleep<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let result = backend.exec("sleep thinker.process --output json").await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("sleeping"))),
        "expected sleeping outcome in {outcomes:?}"
    );

    Ok(())
}

pub(crate) async fn guidebook<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let result = backend
        .exec("guidebook thinker.process --output json")
        .await?;
    let outcomes = result.as_array().expect("expected array of outcomes");

    assert!(
        outcomes
            .iter()
            .any(|o| o.get("type") == Some(&serde_json::json!("guidebook"))),
        "expected guidebook outcome in {outcomes:?}"
    );

    Ok(())
}
