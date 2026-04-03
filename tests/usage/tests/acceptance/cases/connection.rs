use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: init project + persona + texture + nature + agent + two cognitions to connect.
async fn with_connectable_entities<B: Backend>()
-> Result<(Harness<B>, String, String), Box<dyn core::error::Error>> {
    let harness = Harness::<B>::init_project().await?;
    harness
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    harness
        .exec_json("texture set observation --description 'Observations'")
        .await?;
    harness
        .exec_json("nature set caused --description 'One thought produced another'")
        .await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    let first_response = harness
        .exec_json("cognition add thinker.process observation 'First thought'")
        .await?;
    let second_response = harness
        .exec_json("cognition add thinker.process observation 'Second thought'")
        .await?;

    let first_ref = match first_response {
        Responses::Cognition(CognitionResponse::CognitionAdded(wrapped)) => wrapped
            .meta
            .and_then(|m| m.ref_token)
            .expect("expected ref_token in meta for first cognition")
            .to_string(),
        other => panic!("expected CognitionAdded for first cognition, got {other:#?}"),
    };

    let second_ref = match second_response {
        Responses::Cognition(CognitionResponse::CognitionAdded(wrapped)) => wrapped
            .meta
            .and_then(|m| m.ref_token)
            .expect("expected ref_token in meta for second cognition")
            .to_string(),
        other => panic!("expected CognitionAdded for second cognition, got {other:#?}"),
    };

    Ok((harness, first_ref, second_ref))
}

pub(crate) async fn create<B: Backend>() -> TestResult {
    let (harness, from_ref, to_ref) = with_connectable_entities::<B>().await?;

    let cmd = format!("connection create caused {from_ref} {to_ref}");
    let response = harness.exec_json(&cmd).await?;

    assert!(
        matches!(
            response,
            Responses::Connection(ConnectionResponse::ConnectionCreated(_))
        ),
        "expected ConnectionCreated, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("connection list").await?;

    assert!(
        matches!(
            response,
            Responses::Connection(ConnectionResponse::NoConnections)
        ),
        "expected NoConnections, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let (harness, from_ref, to_ref) = with_connectable_entities::<B>().await?;

    let cmd = format!("connection create caused {from_ref} {to_ref}");
    harness.exec_json(&cmd).await?;

    let response = harness.exec_json("connection list").await?;

    match response {
        Responses::Connection(ConnectionResponse::Connections(connections)) => {
            assert_eq!(connections.len(), 1);
        }
        other => panic!("expected Connections, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let (harness, from_ref, to_ref) = with_connectable_entities::<B>().await?;

    let create_cmd = format!("connection create caused {from_ref} {to_ref}");
    let create_response = harness.exec_json(&create_cmd).await?;

    let id = match create_response {
        Responses::Connection(ConnectionResponse::ConnectionCreated(result)) => result.data.id,
        other => panic!("expected ConnectionCreated, got {other:#?}"),
    };

    let show_response = harness.exec_json(&format!("connection show {id}")).await?;

    match show_response {
        Responses::Connection(ConnectionResponse::ConnectionDetails(connection)) => {
            assert_eq!(connection.data.nature.as_str(), "caused");
        }
        other => panic!("expected ConnectionDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn remove_by_id<B: Backend>() -> TestResult {
    let (harness, from_ref, to_ref) = with_connectable_entities::<B>().await?;

    let create_cmd = format!("connection create caused {from_ref} {to_ref}");
    let create_response = harness.exec_json(&create_cmd).await?;

    let id = match create_response {
        Responses::Connection(ConnectionResponse::ConnectionCreated(result)) => result.data.id,
        other => panic!("expected ConnectionCreated, got {other:#?}"),
    };

    let remove_response = harness
        .exec_json(&format!("connection remove {id}"))
        .await?;

    assert!(
        matches!(
            remove_response,
            Responses::Connection(ConnectionResponse::ConnectionRemoved(_))
        ),
        "expected ConnectionRemoved, got {remove_response:?}"
    );

    let list_response = harness.exec_json("connection list").await?;

    assert!(
        matches!(
            list_response,
            Responses::Connection(ConnectionResponse::NoConnections)
        ),
        "expected NoConnections after removal, got {list_response:?}"
    );

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let (harness, first_ref, second_ref) = with_connectable_entities::<B>().await?;

    let response = harness
        .exec_json(&format!(
            "connection create caused {first_ref} {second_ref}"
        ))
        .await?;
    let id = match response {
        Responses::Connection(ConnectionResponse::ConnectionCreated(c)) => c.data.id.to_string(),
        other => panic!("expected ConnectionCreated, got {other:#?}"),
    };

    let prompt = harness
        .exec_prompt(&format!("connection show {id}"))
        .await?;

    assert!(
        !prompt.is_empty(),
        "connection show prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let (harness, first_ref, second_ref) = with_connectable_entities::<B>().await?;
    harness
        .exec_json(&format!(
            "connection create caused {first_ref} {second_ref}"
        ))
        .await?;

    let prompt = harness.exec_prompt("connection list").await?;

    assert!(
        !prompt.is_empty(),
        "connection list prompt should not be empty when connections exist"
    );

    Ok(())
}

pub(crate) async fn remove_prompt<B: Backend>() -> TestResult {
    let (harness, first_ref, second_ref) = with_connectable_entities::<B>().await?;

    let response = harness
        .exec_json(&format!(
            "connection create caused {first_ref} {second_ref}"
        ))
        .await?;
    let id = match response {
        Responses::Connection(ConnectionResponse::ConnectionCreated(c)) => c.data.id.to_string(),
        other => panic!("expected ConnectionCreated, got {other:#?}"),
    };

    let prompt = harness
        .exec_prompt(&format!("connection remove {id}"))
        .await?;

    assert!(
        !prompt.is_empty(),
        "connection remove prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn create_prompt_confirms_creation<B: Backend>() -> TestResult {
    let (harness, first_ref, second_ref) = with_connectable_entities::<B>().await?;

    let prompt = harness
        .exec_prompt(&format!(
            "connection create caused {first_ref} {second_ref}"
        ))
        .await?;

    assert!(
        !prompt.is_empty(),
        "connection create prompt should not be empty — confirm what was recorded"
    );
    assert!(
        prompt.contains("ref:"),
        "connection create prompt should contain a ref token for the created connection"
    );

    Ok(())
}
