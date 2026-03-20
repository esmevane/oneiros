use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: bootstrap with persona + agent so cognitions have an agent to reference.
async fn setup_with_agent<B: Backend>(backend: &mut B) -> TestResult {
    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;
    backend
        .exec("persona set process --description 'Process agents'")
        .await?;
    backend
        .exec("texture set observation --description 'Observations'")
        .await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;
    Ok(())
}

pub(crate) async fn add_creates_cognition<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent(&mut backend).await?;

    let response = backend
        .exec("cognition add thinker.process observation 'A test thought'")
        .await?;

    assert!(
        matches!(
            response.data,
            Responses::Cognition(CognitionResponse::CognitionAdded(_))
        ),
        "expected CognitionAdded, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent(&mut backend).await?;

    let response = backend.exec("cognition list").await?;

    assert!(
        matches!(
            response.data,
            Responses::Cognition(CognitionResponse::NoCognitions)
        ),
        "expected NoCognitions, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent(&mut backend).await?;

    backend
        .exec("cognition add thinker.process observation 'First thought'")
        .await?;
    backend
        .exec("cognition add thinker.process observation 'Second thought'")
        .await?;

    let response = backend.exec("cognition list").await?;

    match response.data {
        Responses::Cognition(CognitionResponse::Cognitions(cognitions)) => {
            assert_eq!(cognitions.len(), 2);
        }
        other => panic!("expected Cognitions, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_filters_by_agent<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent(&mut backend).await?;

    backend
        .exec("agent create other process --description 'Other agent'")
        .await?;

    backend
        .exec("cognition add thinker.process observation 'Thinker thought'")
        .await?;
    backend
        .exec("cognition add other.process observation 'Other thought'")
        .await?;

    let response = backend
        .exec("cognition list --agent thinker.process")
        .await?;

    match response.data {
        Responses::Cognition(CognitionResponse::Cognitions(cognitions)) => {
            assert_eq!(cognitions.len(), 1);
        }
        other => panic!("expected Cognitions, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent(&mut backend).await?;

    let add_response = backend
        .exec("cognition add thinker.process observation 'Show me this'")
        .await?;

    let id = match add_response.data {
        Responses::Cognition(CognitionResponse::CognitionAdded(result)) => result.id,
        other => panic!("expected CognitionAdded, got {other:#?}"),
    };

    let show_cmd = format!("cognition show {id}");
    let show_response = backend.exec(&show_cmd).await?;

    match show_response.data {
        Responses::Cognition(CognitionResponse::CognitionDetails(cognition)) => {
            assert_eq!(cognition.content.as_str(), "Show me this");
        }
        other => panic!("expected CognitionDetails, got {other:#?}"),
    }

    Ok(())
}
