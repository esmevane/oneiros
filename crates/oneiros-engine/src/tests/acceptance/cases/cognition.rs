use super::*;

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
            response,
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
            response,
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

    match response {
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

    match response {
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

    let id = match add_response {
        Responses::Cognition(CognitionResponse::CognitionAdded(result)) => result.data.id(),
        other => panic!("expected CognitionAdded, got {other:#?}"),
    };

    let show_response = harness.exec_json(&format!("cognition show {id}")).await?;

    match show_response {
        Responses::Cognition(CognitionResponse::CognitionDetails(cognition)) => {
            assert_eq!(cognition.data.content().as_str(), "Show me this");
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
    let id = match response {
        Responses::Cognition(CognitionResponse::CognitionAdded(c)) => c.data.id().to_string(),
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

pub(crate) async fn show_json_includes_ref<B: Backend>() -> TestResult {
    let harness = with_agent::<B>().await?;

    let add_response = harness
        .exec_json("cognition add thinker.process observation 'Show me this'")
        .await?;

    let id = match add_response {
        Responses::Cognition(CognitionResponse::CognitionAdded(result)) => result.data.id(),
        other => panic!("expected CognitionAdded, got {other:#?}"),
    };

    let show_response = harness.exec_json(&format!("cognition show {id}")).await?;

    match show_response {
        Responses::Cognition(CognitionResponse::CognitionDetails(wrapped)) => {
            let ref_token = wrapped
                .meta
                .and_then(|m| m.ref_token)
                .expect("show response should include ref_token in entity meta");
            assert!(
                ref_token.to_string().starts_with("ref:"),
                "ref_token should be a valid ref token"
            );
        }
        other => panic!("expected CognitionDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_json_items_include_refs<B: Backend>() -> TestResult {
    let harness = with_agent::<B>().await?;

    harness
        .exec_json("cognition add thinker.process observation 'First thought'")
        .await?;
    harness
        .exec_json("cognition add thinker.process observation 'Second thought'")
        .await?;

    let response = harness.exec_json("cognition list").await?;
    let json = serde_json::to_value(&response)?;

    let items = json["data"]["items"]
        .as_array()
        .expect("cognition list should have data.items array");

    assert_eq!(items.len(), 2);

    for item in items {
        let ref_token = item["meta"]["ref_token"]
            .as_str()
            .expect("each listed cognition should have meta.ref_token");
        assert!(
            ref_token.starts_with("ref:"),
            "ref_token should start with 'ref:', got: {ref_token}"
        );
    }

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
