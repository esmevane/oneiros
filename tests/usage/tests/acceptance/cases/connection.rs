use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: bootstrap with agent + nature + two cognitions to connect.
async fn setup_with_connectable_entities<B: Backend>(
    backend: &mut B,
) -> Result<(String, String), Box<dyn core::error::Error>> {
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
        .exec("nature set caused --description 'One thought produced another'")
        .await?;
    backend
        .exec("agent create thinker process --description 'A thinking agent'")
        .await?;

    // Create two cognitions to connect
    let first_response = backend
        .exec("cognition add thinker.process observation 'First thought'")
        .await?;
    let second_response = backend
        .exec("cognition add thinker.process observation 'Second thought'")
        .await?;

    let first_ref = match first_response.data {
        Responses::Cognition(CognitionResponse::CognitionAdded(result)) => {
            result.ref_token.to_string()
        }
        other => panic!("expected CognitionAdded for first cognition, got {other:#?}"),
    };

    let second_ref = match second_response.data {
        Responses::Cognition(CognitionResponse::CognitionAdded(result)) => {
            result.ref_token.to_string()
        }
        other => panic!("expected CognitionAdded for second cognition, got {other:#?}"),
    };

    Ok((first_ref, second_ref))
}

pub(crate) async fn create<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    let (from_ref, to_ref) = setup_with_connectable_entities(&mut backend).await?;

    let cmd = format!("connection create caused {from_ref} {to_ref}");
    let response = backend.exec(&cmd).await?;

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

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    let response = backend.exec("connection list").await?;

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
    backend.exec(&cmd).await?;

    let response = backend.exec("connection list").await?;

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
    let create_response = backend.exec(&create_cmd).await?;

    let id = match create_response.data {
        Responses::Connection(ConnectionResponse::ConnectionCreated(result)) => result.id,
        other => panic!("expected ConnectionCreated, got {other:#?}"),
    };

    let show_cmd = format!("connection show {id}");
    let show_response = backend.exec(&show_cmd).await?;

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
    let create_response = backend.exec(&create_cmd).await?;

    let id = match create_response.data {
        Responses::Connection(ConnectionResponse::ConnectionCreated(result)) => result.id,
        other => panic!("expected ConnectionCreated, got {other:#?}"),
    };

    let remove_cmd = format!("connection remove {id}");
    let remove_response = backend.exec(&remove_cmd).await?;

    assert!(
        matches!(
            remove_response.data,
            Responses::Connection(ConnectionResponse::ConnectionRemoved(_))
        ),
        "expected ConnectionRemoved, got {remove_response:?}"
    );

    // Verify it's gone
    let list_response = backend.exec("connection list").await?;

    assert!(
        matches!(
            list_response.data,
            Responses::Connection(ConnectionResponse::NoConnections)
        ),
        "expected NoConnections after removal, got {list_response:?}"
    );

    Ok(())
}
