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
        Responses::Json(v) => {
            assert_eq!(
                v.get("type").and_then(|t| t.as_str()),
                Some("status"),
                "expected type=status, got {v:?}"
            );
        }
        other => panic!("expected Json(status), got {other:#?}"),
    }

    Ok(())
}
