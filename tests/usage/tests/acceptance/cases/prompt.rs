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

pub(crate) async fn dream_prompt_contains_identity<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let prompt = backend.exec_prompt("dream thinker.process").await?;

    assert!(
        prompt.contains("thinker.process"),
        "dream prompt should contain agent name, got: {}",
        &prompt[..prompt.len().min(200)]
    );
    assert!(
        prompt.contains("## Your Identity"),
        "dream prompt should have identity section"
    );
    assert!(
        prompt.contains("## Instructions"),
        "dream prompt should have instructions section"
    );

    Ok(())
}

pub(crate) async fn dream_prompt_contains_vocabulary<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let prompt = backend.exec_prompt("dream thinker.process").await?;

    assert!(
        prompt.contains("## Cognitive Textures"),
        "dream prompt should include textures section"
    );
    assert!(
        prompt.contains("observation"),
        "dream prompt should include seeded texture"
    );
    assert!(
        prompt.contains("## Memory Levels"),
        "dream prompt should include levels section"
    );

    Ok(())
}

pub(crate) async fn dream_prompt_contains_memories<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    backend
        .exec_json("memory add thinker.process core 'I remember everything'")
        .await?;

    let prompt = backend.exec_prompt("dream thinker.process").await?;

    assert!(
        prompt.contains("## Your Memories"),
        "dream prompt should have memories section when memories exist"
    );
    assert!(
        prompt.contains("I remember everything"),
        "dream prompt should contain memory content"
    );

    Ok(())
}

pub(crate) async fn dream_prompt_contains_cognitions<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    backend
        .exec_json("cognition add thinker.process observation 'Something interesting happened'")
        .await?;

    let prompt = backend.exec_prompt("dream thinker.process").await?;

    assert!(
        prompt.contains("## Your Cognitions"),
        "dream prompt should have cognitions section when cognitions exist"
    );
    assert!(
        prompt.contains("Something interesting happened"),
        "dream prompt should contain cognition content"
    );

    Ok(())
}

pub(crate) async fn introspect_prompt_contains_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let prompt = backend.exec_prompt("introspect thinker.process").await?;

    assert!(
        prompt.contains("thinker.process"),
        "introspect prompt should contain agent name"
    );
    assert!(
        prompt.contains("Before your context compacts"),
        "introspect prompt should contain introspection instructions"
    );

    Ok(())
}

pub(crate) async fn reflect_prompt_contains_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let prompt = backend.exec_prompt("reflect thinker.process").await?;

    assert!(
        prompt.contains("thinker.process"),
        "reflect prompt should contain agent name"
    );
    assert!(
        prompt.contains("Something significant"),
        "reflect prompt should contain reflection instructions"
    );

    Ok(())
}

pub(crate) async fn guidebook_prompt_contains_capabilities<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let prompt = backend.exec_prompt("guidebook thinker.process").await?;

    assert!(
        prompt.contains("Cognitive Guidebook"),
        "guidebook prompt should have title"
    );
    assert!(
        prompt.contains("thinker.process"),
        "guidebook prompt should contain agent name"
    );
    assert!(
        prompt.contains("Your Lifecycle"),
        "guidebook prompt should contain lifecycle section"
    );
    assert!(
        prompt.contains("Your Agency"),
        "guidebook prompt should contain agency section"
    );

    Ok(())
}

pub(crate) async fn wake_prompt_contains_identity<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_seeded_agent(&mut backend).await?;

    let prompt = backend.exec_prompt("wake thinker.process").await?;

    assert!(
        prompt.contains("thinker.process"),
        "wake prompt should contain agent name"
    );
    assert!(
        prompt.contains("## Your Identity"),
        "wake prompt should have identity section (same template as dream)"
    );

    Ok(())
}
