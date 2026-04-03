use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn status_shows_agents<B: Backend>() -> TestResult {
    let harness = Harness::<B>::seed_project().await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    let prompt = harness.exec_prompt("status").await?;

    assert!(!prompt.is_empty(), "status prompt should not be empty");
    assert!(
        prompt.contains("thinker.process"),
        "status prompt should contain the agent name"
    );
    assert!(
        prompt.contains("Cog"),
        "status prompt should contain column headers"
    );

    Ok(())
}

pub(crate) async fn returns_activity_table<B: Backend>() -> TestResult {
    let harness = Harness::<B>::seed_project().await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    let response = harness.exec_json("status").await?;

    match &response.data {
        Responses::Continuity(ContinuityResponse::Status(table)) => {
            assert!(
                !table.agents.is_empty(),
                "activity table should contain agents"
            );
            assert!(
                table
                    .agents
                    .iter()
                    .any(|a| a.name == AgentName::new("thinker.process")),
                "activity table should include the created agent"
            );
        }
        other => panic!("expected Continuity(Status), got {other:#?}"),
    }

    Ok(())
}
