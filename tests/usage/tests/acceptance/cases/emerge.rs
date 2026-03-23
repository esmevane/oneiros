use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn creates_and_wakes_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend.exec_json("seed core").await?;

    let response = backend
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
    let show = backend.exec_json("agent show newborn.process").await?;

    assert!(
        matches!(show.data, Responses::Agent(AgentResponse::AgentDetails(_))),
        "expected AgentDetails after emerge, got {show:#?}"
    );

    Ok(())
}

pub(crate) async fn recede_retires_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend.exec_json("seed core").await?;

    backend
        .exec_json("agent create retiring process --description 'Will retire'")
        .await?;

    let response = backend.exec_json("recede retiring.process").await?;

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
