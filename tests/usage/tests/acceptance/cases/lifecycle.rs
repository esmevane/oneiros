use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: bootstrap with seeded vocabulary + an agent.
async fn setup_with_seeded_agent<B: Backend>(backend: &mut B) -> TestResult {
    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend.exec_json("seed core").await?;
    backend
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;
    Ok(())
}

pub(crate) async fn wake<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec_json("wake thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Continuity(ContinuityResponse::Waking(_))
        ),
        "expected Waking, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn dream<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec_json("dream thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Continuity(ContinuityResponse::Dreaming(_))
        ),
        "expected Dreaming, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn introspect<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec_json("introspect thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Continuity(ContinuityResponse::Introspecting(_))
        ),
        "expected Introspecting, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn reflect<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec_json("reflect thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Continuity(ContinuityResponse::Reflecting(_))
        ),
        "expected Reflecting, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn sleep<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec_json("sleep thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Continuity(ContinuityResponse::Sleeping(_))
        ),
        "expected Sleeping, got {response:#?}"
    );

    Ok(())
}

/// Dream context should include vocabulary (textures, levels, sensations, natures, urges),
/// connections, and pressure readings — not just agent + cognitions + memories + experiences.
pub(crate) async fn dream_includes_vocabulary_and_connections<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    // Add a cognition so we have something to connect
    backend
        .exec_json("cognition add thinker.process observation 'First thought'")
        .await?;

    let response = backend.exec_json("dream thinker.process").await?;

    match response.data {
        Responses::Continuity(ContinuityResponse::Dreaming(ctx)) => {
            // The dream context should have the seeded vocabulary
            let json = serde_json::to_value(&ctx).unwrap();
            let obj = json.as_object().unwrap();

            // These fields should exist and be non-empty (seed core creates them)
            assert!(
                obj.contains_key("textures"),
                "dream context should include textures, got keys: {:?}",
                obj.keys().collect::<Vec<_>>()
            );
            assert!(
                obj.contains_key("levels"),
                "dream context should include levels"
            );
            assert!(
                obj.contains_key("sensations"),
                "dream context should include sensations"
            );
            assert!(
                obj.contains_key("natures"),
                "dream context should include natures"
            );
            assert!(
                obj.contains_key("urges"),
                "dream context should include urges"
            );
        }
        other => panic!("expected Dreaming, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn guidebook<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let response = backend.exec_json("guidebook thinker.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Continuity(ContinuityResponse::Guidebook(_))
        ),
        "expected Guidebook, got {response:#?}"
    );

    Ok(())
}
