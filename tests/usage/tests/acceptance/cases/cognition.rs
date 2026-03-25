use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: init project + persona + texture + agent for cognition tests.
async fn with_agent<B: Backend>() -> Result<Harness<B>, Box<dyn core::error::Error>> {
    let harness = Harness::<B>::init_project().await?;
    harness
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    harness
        .exec_json("texture set observation --description 'Observations'")
        .await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;
    Ok(harness)
}

pub(crate) async fn add_creates_cognition<B: Backend>() -> TestResult {
    let harness = with_agent::<B>().await?;

    let response = harness
        .exec_json("cognition add thinker.process observation 'A test thought'")
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
    let harness = with_agent::<B>().await?;

    let response = harness.exec_json("cognition list").await?;

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
    let harness = with_agent::<B>().await?;

    harness
        .exec_json("cognition add thinker.process observation 'First thought'")
        .await?;
    harness
        .exec_json("cognition add thinker.process observation 'Second thought'")
        .await?;

    let response = harness.exec_json("cognition list").await?;

    match response.data {
        Responses::Cognition(CognitionResponse::Cognitions(cognitions)) => {
            assert_eq!(cognitions.len(), 2);
        }
        other => panic!("expected Cognitions, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_filters_by_agent<B: Backend>() -> TestResult {
    let harness = with_agent::<B>().await?;

    harness
        .exec_json("agent create other process --description 'Other agent'")
        .await?;

    harness
        .exec_json("cognition add thinker.process observation 'Thinker thought'")
        .await?;
    harness
        .exec_json("cognition add other.process observation 'Other thought'")
        .await?;

    let response = harness
        .exec_json("cognition list --agent thinker.process")
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
    let harness = with_agent::<B>().await?;

    let add_response = harness
        .exec_json("cognition add thinker.process observation 'Show me this'")
        .await?;

    let id = match add_response.data {
        Responses::Cognition(CognitionResponse::CognitionAdded(result)) => result.id,
        other => panic!("expected CognitionAdded, got {other:#?}"),
    };

    let show_response = harness.exec_json(&format!("cognition show {id}")).await?;

    match show_response.data {
        Responses::Cognition(CognitionResponse::CognitionDetails(cognition)) => {
            assert_eq!(cognition.content.as_str(), "Show me this");
        }
        other => panic!("expected CognitionDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let harness = with_agent::<B>().await?;

    let response = harness
        .exec_json("cognition add thinker.process observation 'Show me this'")
        .await?;
    let id = match response.data {
        Responses::Cognition(CognitionResponse::CognitionAdded(c)) => c.id.to_string(),
        other => panic!("expected CognitionAdded, got {other:#?}"),
    };

    let prompt = harness.exec_prompt(&format!("cognition show {id}")).await?;

    assert!(
        !prompt.is_empty(),
        "cognition show prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = with_agent::<B>().await?;
    harness
        .exec_json("cognition add thinker.process observation 'A thought'")
        .await?;

    let prompt = harness
        .exec_prompt("cognition list --agent thinker.process")
        .await?;

    assert!(
        !prompt.is_empty(),
        "cognition list prompt should not be empty when cognitions exist"
    );

    Ok(())
}

pub(crate) async fn add_prompt_confirms_creation<B: Backend>() -> TestResult {
    let harness = with_agent::<B>().await?;

    let prompt = harness
        .exec_prompt("cognition add thinker.process observation 'A prompted thought'")
        .await?;

    assert!(
        !prompt.is_empty(),
        "cognition add prompt should not be empty — confirm what was recorded"
    );
    assert!(
        prompt.contains("ref:"),
        "cognition add prompt should contain a ref token for the created cognition"
    );

    Ok(())
}
