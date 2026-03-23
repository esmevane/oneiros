use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: bootstrap with agent + nature + two cognitions to connect.
async fn setup_with_connectable_entities<B: Backend>(
    backend: &mut B,
) -> Result<(String, String), Box<dyn core::error::Error>> {
    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    backend
        .exec_json("texture set observation --description 'Observations'")
        .await?;
    backend
        .exec_json("nature set caused --description 'One thought produced another'")
        .await?;
    backend
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    // Create two cognitions to connect
    let first_response = backend
        .exec_json("cognition add thinker.process observation 'First thought'")
        .await?;
    let second_response = backend
        .exec_json("cognition add thinker.process observation 'Second thought'")
        .await?;

    assert!(
        matches!(
            first_response.data,
            Responses::Cognition(CognitionResponse::CognitionAdded(_))
        ),
        "expected CognitionAdded for first cognition, got {:#?}",
        first_response.data
    );
    let first_ref = first_response
        .meta
        .and_then(|m| m.ref_token)
        .expect("expected ref_token in meta for first cognition")
        .to_string();

    assert!(
        matches!(
            second_response.data,
            Responses::Cognition(CognitionResponse::CognitionAdded(_))
        ),
        "expected CognitionAdded for second cognition, got {:#?}",
        second_response.data
    );
    let second_ref = second_response
        .meta
        .and_then(|m| m.ref_token)
        .expect("expected ref_token in meta for second cognition")
        .to_string();

    Ok((first_ref, second_ref))
}

pub(crate) async fn create<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    let (from_ref, to_ref) = setup_with_connectable_entities(&mut backend).await?;

    let cmd = format!("connection create caused {from_ref} {to_ref}");
    let response = backend.exec_json(&cmd).await?;

    assert!(
        matches!(
            response.data,
            Responses::Connection(ConnectionResponse::ConnectionCreated(_))
        ),
        "expected ConnectionCreated, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;

    let response = backend.exec_json("connection list").await?;

    assert!(
        matches!(
            response.data,
            Responses::Connection(ConnectionResponse::NoConnections)
        ),
        "expected NoConnections, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    let (from_ref, to_ref) = setup_with_connectable_entities(&mut backend).await?;

    let cmd = format!("connection create caused {from_ref} {to_ref}");
    backend.exec_json(&cmd).await?;

    let response = backend.exec_json("connection list").await?;

    match response.data {
        Responses::Connection(ConnectionResponse::Connections(connections)) => {
            assert_eq!(connections.len(), 1);
        }
        other => panic!("expected Connections, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    let (from_ref, to_ref) = setup_with_connectable_entities(&mut backend).await?;

    let create_cmd = format!("connection create caused {from_ref} {to_ref}");
    let create_response = backend.exec_json(&create_cmd).await?;

    let id = match create_response.data {
        Responses::Connection(ConnectionResponse::ConnectionCreated(result)) => result.id,
        other => panic!("expected ConnectionCreated, got {other:#?}"),
    };

    let show_cmd = format!("connection show {id}");
    let show_response = backend.exec_json(&show_cmd).await?;

    match show_response.data {
        Responses::Connection(ConnectionResponse::ConnectionDetails(connection)) => {
            assert_eq!(connection.nature.as_str(), "caused");
        }
        other => panic!("expected ConnectionDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn remove_by_id<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    let (from_ref, to_ref) = setup_with_connectable_entities(&mut backend).await?;

    let create_cmd = format!("connection create caused {from_ref} {to_ref}");
    let create_response = backend.exec_json(&create_cmd).await?;

    let id = match create_response.data {
        Responses::Connection(ConnectionResponse::ConnectionCreated(result)) => result.id,
        other => panic!("expected ConnectionCreated, got {other:#?}"),
    };

    let remove_cmd = format!("connection remove {id}");
    let remove_response = backend.exec_json(&remove_cmd).await?;

    assert!(
        matches!(
            remove_response.data,
            Responses::Connection(ConnectionResponse::ConnectionRemoved(_))
        ),
        "expected ConnectionRemoved, got {remove_response:?}"
    );

    // Verify it's gone
    let list_response = backend.exec_json("connection list").await?;

    assert!(
        matches!(
            list_response.data,
            Responses::Connection(ConnectionResponse::NoConnections)
        ),
        "expected NoConnections after removal, got {list_response:?}"
    );

    Ok(())
}
