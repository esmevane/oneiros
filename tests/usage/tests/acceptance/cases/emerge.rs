use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn creates_and_wakes_agent<B: Backend>() -> TestResult {
    let harness = Harness::<B>::seed_project().await?;

    let response = harness
        .exec_json("emerge newborn process --description 'A new agent'")
        .await?;

    match &response.data {
        // Engine: typed continuity response
        Responses::Continuity(ContinuityResponse::Emerged(ctx)) => {
            assert_eq!(ctx.agent.name, AgentName::new("newborn.process"));
        }
        // Legacy: inline JSON (will be removed when legacy is retired)
        Responses::Json(v) => {
            assert_eq!(v.get("type").and_then(|t| t.as_str()), Some("emerged"));
        }
        other => panic!("expected Continuity(Emerged) or Json(emerged), got {other:#?}"),
    }

    // Verify the agent exists via typed response
    let show = harness.exec_json("agent show newborn.process").await?;

    assert!(
        matches!(show.data, Responses::Agent(AgentResponse::AgentDetails(_))),
        "expected AgentDetails after emerge, got {show:#?}"
    );

    Ok(())
}

pub(crate) async fn emerge_prompt_contains_identity<B: Backend>() -> TestResult {
    let harness = Harness::<B>::seed_project().await?;

    let prompt = harness
        .exec_prompt("emerge thinker process --description 'A thinking agent'")
        .await?;

    assert!(
        prompt.contains("thinker.process"),
        "emerge prompt should contain the created agent's full name"
    );
    assert!(
        prompt.contains("## Your Identity"),
        "emerge prompt should render the dream template for the new agent"
    );

    Ok(())
}

pub(crate) async fn recede_prompt_contains_agent<B: Backend>() -> TestResult {
    let harness = Harness::<B>::seed_project().await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    let prompt = harness.exec_prompt("recede thinker.process").await?;

    assert!(
        !prompt.is_empty(),
        "recede prompt should not be empty — the agent needs a farewell"
    );
    assert!(
        prompt.contains("thinker.process"),
        "recede prompt should contain the agent name"
    );

    Ok(())
}

pub(crate) async fn recede_retires_agent<B: Backend>() -> TestResult {
    let harness = Harness::<B>::seed_project().await?;

    harness
        .exec_json("agent create retiring process --description 'Will retire'")
        .await?;

    let response = harness.exec_json("recede retiring.process").await?;

    match &response.data {
        // Engine: typed continuity response
        Responses::Continuity(ContinuityResponse::Receded(name)) => {
            assert_eq!(*name, AgentName::new("retiring.process"));
        }
        // Legacy: inline JSON (will be removed when legacy is retired)
        Responses::Json(v) => {
            assert_eq!(v.get("type").and_then(|t| t.as_str()), Some("receded"));
        }
        other => panic!("expected Continuity(Receded) or Json(receded), got {other:#?}"),
    }

    Ok(())
}
