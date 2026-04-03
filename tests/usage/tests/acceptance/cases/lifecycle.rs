use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: bootstrap with seeded vocabulary + an agent.
async fn with_seeded_agent<B: Backend>() -> Result<Harness<B>, Box<dyn core::error::Error>> {
    let harness = Harness::<B>::seed_project().await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await
        .map_err(|e| -> Box<dyn core::error::Error> { e.to_string().into() })?;
    Ok(harness)
}

pub(crate) async fn wake<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    let response = harness.exec_json("wake thinker.process").await?;

    assert!(
        matches!(
            response,
            Responses::Continuity(ContinuityResponse::Waking(_))
        ),
        "expected Waking, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn dream<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    let response = harness.exec_json("dream thinker.process").await?;

    assert!(
        matches!(
            response,
            Responses::Continuity(ContinuityResponse::Dreaming(_))
        ),
        "expected Dreaming, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn introspect<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    let response = harness.exec_json("introspect thinker.process").await?;

    assert!(
        matches!(
            response,
            Responses::Continuity(ContinuityResponse::Introspecting(_))
        ),
        "expected Introspecting, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn reflect<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    let response = harness.exec_json("reflect thinker.process").await?;

    assert!(
        matches!(
            response,
            Responses::Continuity(ContinuityResponse::Reflecting(_))
        ),
        "expected Reflecting, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn sleep<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    let response = harness.exec_json("sleep thinker.process").await?;

    assert!(
        matches!(
            response,
            Responses::Continuity(ContinuityResponse::Sleeping(_))
        ),
        "expected Sleeping, got {response:#?}"
    );

    Ok(())
}

/// Dream context should include vocabulary (textures, levels, sensations, natures, urges),
/// connections, and pressure readings — not just agent + cognitions + memories + experiences.
pub(crate) async fn dream_includes_vocabulary_and_connections<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    // Add a cognition so we have something to connect
    harness
        .exec_json("cognition add thinker.process observation 'First thought'")
        .await?;

    let response = harness.exec_json("dream thinker.process").await?;

    match response {
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
    let harness = with_seeded_agent::<B>().await?;

    let response = harness.exec_json("guidebook thinker.process").await?;

    assert!(
        matches!(
            response,
            Responses::Continuity(ContinuityResponse::Guidebook(_))
        ),
        "expected Guidebook, got {response:#?}"
    );

    Ok(())
}

// -- Compact dream prompt tests --

pub(crate) async fn dream_prompt_vocabulary_names_only<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    // Set a texture with a rich prompt so we can verify it's NOT shown in compact dream
    harness
        .exec_json("texture set observation --description 'Noticing things' --prompt 'When you notice something interesting about the code, architecture, or process, capture it as an observation. Focus on what you see, not what to do about it.'")
        .await?;

    let prompt = harness.exec_prompt("dream thinker.process").await?;

    // Should have vocabulary names
    assert!(
        prompt.contains("observation"),
        "dream prompt should list vocabulary names"
    );
    // Should NOT have full prompts inline
    assert!(
        !prompt.contains("Focus on what you see, not what to do about it"),
        "dream prompt should not include full vocabulary prompts"
    );
    // Should reference guidebook
    assert!(
        prompt.contains("guidebook"),
        "dream prompt should point to guidebook for vocabulary details"
    );

    Ok(())
}

pub(crate) async fn dream_prompt_non_core_memories_summarized<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    harness
        .exec_json("memory add thinker.process core 'I am the core of all things'")
        .await?;
    harness
        .exec_json("memory add thinker.process project 'A detailed project memory with extensive architectural context that describes the full system design and all the patterns we discovered along the way'")
        .await?;

    let prompt = harness.exec_prompt("dream thinker.process").await?;

    // Core memory should be fully inline
    assert!(
        prompt.contains("I am the core of all things"),
        "core memory should be fully inline"
    );
    // Non-core should be truncated
    assert!(
        !prompt.contains("all the patterns we discovered along the way"),
        "non-core memory should be truncated in compact dream"
    );
    // Should have ref tokens
    assert!(
        prompt.contains("ref:"),
        "compact dream should include ref tokens for summarized memories"
    );

    Ok(())
}

pub(crate) async fn dream_deep_prompt_includes_full_vocabulary<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    // Set a texture with a rich prompt
    harness
        .exec_json("texture set observation --description 'Noticing things' --prompt 'When you notice something interesting about the code, architecture, or process, capture it as an observation. Focus on what you see, not what to do about it.'")
        .await?;

    let prompt = harness.exec_prompt("dream thinker.process --deep").await?;

    // Deep mode should include full texture prompts
    assert!(
        prompt.contains("Focus on what you see, not what to do about it"),
        "deep dream should include full texture prompts"
    );

    Ok(())
}

// -- Prompt output tests --

pub(crate) async fn dream_prompt_contains_identity<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    let prompt = harness.exec_prompt("dream thinker.process").await?;

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
    let harness = with_seeded_agent::<B>().await?;

    let prompt = harness.exec_prompt("dream thinker.process").await?;

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
    let harness = with_seeded_agent::<B>().await?;

    harness
        .exec_json("memory add thinker.process core 'I remember everything'")
        .await?;

    let prompt = harness.exec_prompt("dream thinker.process").await?;

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
    let harness = with_seeded_agent::<B>().await?;

    harness
        .exec_json("cognition add thinker.process observation 'Something interesting happened'")
        .await?;

    let prompt = harness.exec_prompt("dream thinker.process").await?;

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
    let harness = with_seeded_agent::<B>().await?;

    let prompt = harness.exec_prompt("introspect thinker.process").await?;

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
    let harness = with_seeded_agent::<B>().await?;

    let prompt = harness.exec_prompt("reflect thinker.process").await?;

    assert!(
        prompt.contains("thinker.process"),
        "reflect prompt should contain agent name"
    );
    assert!(
        prompt.contains("Something shifted"),
        "reflect prompt should contain reflection instructions"
    );

    Ok(())
}

pub(crate) async fn guidebook_prompt_contains_capabilities<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    let prompt = harness.exec_prompt("guidebook thinker.process").await?;

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
    let harness = with_seeded_agent::<B>().await?;

    let prompt = harness.exec_prompt("wake thinker.process").await?;

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

pub(crate) async fn sleep_prompt_contains_agent<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    let prompt = harness.exec_prompt("sleep thinker.process").await?;

    assert!(
        !prompt.is_empty(),
        "sleep prompt should not be empty — the agent needs to know what happened"
    );
    assert!(
        prompt.contains("thinker.process"),
        "sleep prompt should contain the agent name"
    );

    Ok(())
}

pub(crate) async fn sense_prompt_contains_agent<B: Backend>() -> TestResult {
    let harness = with_seeded_agent::<B>().await?;

    let prompt = harness.exec_prompt("sense thinker.process").await?;

    assert!(!prompt.is_empty(), "sense prompt should not be empty");
    assert!(
        prompt.contains("thinker.process"),
        "sense prompt should contain the agent name"
    );

    Ok(())
}
