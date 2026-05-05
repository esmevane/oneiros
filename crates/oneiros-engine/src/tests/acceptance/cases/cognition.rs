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

/// Vertical slice through the lazy bookmark actor tree: `cognition
/// add` dispatches a `Message<AtBookmark>` through the bus, the host
/// actor lazy-spawns the project actor for the brain, the project
/// actor lazy-spawns bookmark + chronicle actors for the bookmark on
/// scope, and the projection materializes the row that the service
/// then fetches eventually-consistently.
///
/// The assertion that proves it: the response carries the projected
/// record, and a follow-up `cognition show` against the projected ID
/// returns the same record. If the bus didn't reach the bookmark
/// actor, the projection would be empty and `show` would 404.
pub(crate) async fn add_dispatches_via_bus<B: Backend>() -> TestResult {
    let harness = with_agent::<B>().await?;

    let response = harness
        .exec_json("cognition add thinker.process observation 'bus-routed thought'")
        .await?;

    let added = match response {
        Responses::Cognition(CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(
            added,
        ))) => added.cognition,
        other => panic!("expected CognitionAdded, got {other:#?}"),
    };

    assert_eq!(added.content.as_str(), "bus-routed thought");

    let shown = harness
        .exec_json(&format!("cognition show {}", added.id))
        .await?;

    match shown {
        Responses::Cognition(CognitionResponse::CognitionDetails(
            CognitionDetailsResponse::V1(details),
        )) => {
            assert_eq!(
                details.cognition.id, added.id,
                "show must return the projected cognition"
            );
            assert_eq!(details.cognition.content.as_str(), "bus-routed thought");
        }
        other => panic!("expected CognitionDetails, got {other:#?}"),
    }

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
        Responses::Cognition(CognitionResponse::Cognitions(CognitionsResponse::V1(cognitions))) => {
            assert_eq!(cognitions.items.len(), 2);
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
        Responses::Cognition(CognitionResponse::Cognitions(CognitionsResponse::V1(cognitions))) => {
            assert_eq!(cognitions.items.len(), 1);
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
        Responses::Cognition(CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(
            added,
        ))) => added.cognition.id,
        other => panic!("expected CognitionAdded, got {other:#?}"),
    };

    let show_response = harness.exec_json(&format!("cognition show {id}")).await?;

    match show_response {
        Responses::Cognition(CognitionResponse::CognitionDetails(
            CognitionDetailsResponse::V1(details),
        )) => {
            assert_eq!(details.cognition.content.as_str(), "Show me this");
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
        Responses::Cognition(CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(
            added,
        ))) => added.cognition.id.to_string(),
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
        Responses::Cognition(CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(
            added,
        ))) => added.cognition.id,
        other => panic!("expected CognitionAdded, got {other:#?}"),
    };

    let show_response = harness.exec_json(&format!("cognition show {id}")).await?;

    match show_response {
        Responses::Cognition(CognitionResponse::CognitionDetails(
            CognitionDetailsResponse::V1(details),
        )) => {
            assert_eq!(details.cognition.id, id);
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

    match response {
        Responses::Cognition(CognitionResponse::Cognitions(CognitionsResponse::V1(listed))) => {
            assert_eq!(listed.items.len(), 2);
            for item in listed.items {
                assert!(!item.id.to_string().is_empty());
            }
        }
        other => panic!("expected Cognitions, got {other:#?}"),
    }

    Ok(())
}

/// `cognition list --query "<text>"` narrows the listing by FTS5 match
/// on cognition content. Same query semantics as `search`, scoped to the
/// cognition kind. Lists ARE searches.
pub(crate) async fn list_filters_by_query<B: Backend>() -> TestResult {
    let harness = with_agent::<B>().await?;

    harness
        .exec_json("cognition add thinker.process observation 'The garden is growing'")
        .await?;
    harness
        .exec_json("cognition add thinker.process observation 'Unrelated content here'")
        .await?;

    let response = harness.exec_json("cognition list --query garden").await?;

    match response {
        Responses::Cognition(CognitionResponse::Cognitions(CognitionsResponse::V1(listed))) => {
            assert_eq!(
                listed.items.len(),
                1,
                "expected 1 cognition matching 'garden'"
            );
            assert_eq!(listed.total, 1);
            assert!(
                listed.items[0].content.as_str().contains("garden"),
                "expected matching cognition's content to contain the query"
            );
        }
        other => panic!("expected Cognitions, got {other:#?}"),
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
