use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn creates_and_wakes_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend.exec("seed core").await?;

    let response = backend
        .exec("emerge newborn process --description 'A new agent'")
        .await?;

    match &response.data {
        Responses::Json(v) => {
            assert_eq!(
                v.get("type").and_then(|t| t.as_str()),
                Some("emerged"),
                "expected type=emerged, got {v:?}"
            );
        }
        other => panic!("expected Json(emerged), got {other:#?}"),
    }

    // Verify the agent exists via typed response
    let show = backend.exec("agent show newborn.process").await?;

    assert!(
        matches!(show.data, Responses::Agent(AgentResponse::AgentDetails(_))),
        "expected AgentDetails after emerge, got {show:#?}"
    );

    Ok(())
}

pub(crate) async fn recede_retires_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend.exec("seed core").await?;

    backend
        .exec("agent create retiring process --description 'Will retire'")
        .await?;

    let response = backend.exec("recede retiring.process").await?;

    match &response.data {
        Responses::Json(v) => {
            assert_eq!(
                v.get("type").and_then(|t| t.as_str()),
                Some("receded"),
                "expected type=receded, got {v:?}"
            );
        }
        other => panic!("expected Json(receded), got {other:#?}"),
    }

    Ok(())
}
