use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn returns_agent_status<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend.exec("seed core").await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;

    let response = backend.exec("status thinker.process").await?;

    match &response.data {
        // Engine: typed continuity response
        Responses::Continuity(ContinuityResponse::Status(ctx)) => {
            assert_eq!(ctx.agent.name, AgentName::new("thinker.process"));
        }
        // Legacy: inline JSON (will be removed when legacy is retired)
        Responses::Json(v) => {
            assert_eq!(v.get("type").and_then(|t| t.as_str()), Some("status"));
        }
        other => panic!("expected Continuity(Status) or Json(status), got {other:#?}"),
    }

    Ok(())
}
