use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: bootstrap + seed a persona so agents can reference it.
async fn setup_with_persona<B: Backend>(backend: &mut B) -> TestResult {
    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    Ok(())
}

pub(crate) async fn create_with_persona<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    let response = backend
        .exec_json("agent create test process --description 'A test agent'")
        .await?;

    assert!(
        matches!(
            response.data,
            Responses::Agent(AgentResponse::AgentCreated(_))
        ),
        "expected AgentCreated, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn show_returns_details<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    backend
        .exec_json("agent create viewer process --description 'Views things'")
        .await?;

    let response = backend.exec_json("agent show viewer.process").await?;

    match response.data {
        Responses::Agent(AgentResponse::AgentDetails(agent)) => {
            assert_eq!(agent.name.as_str(), "viewer.process");
            assert_eq!(agent.persona.as_str(), "process");
        }
        other => panic!("expected AgentDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;

    let response = backend.exec_json("agent list").await?;

    assert!(
        matches!(response.data, Responses::Agent(AgentResponse::NoAgents)),
        "expected NoAgents, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    backend
        .exec_json("agent create first process --description 'First'")
        .await?;
    backend
        .exec_json("agent create second process --description 'Second'")
        .await?;

    let response = backend.exec_json("agent list").await?;

    match response.data {
        Responses::Agent(AgentResponse::Agents(agents)) => {
            assert_eq!(agents.len(), 2);
        }
        other => panic!("expected Agents, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn update_changes_fields<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    backend
        .exec_json("agent create mutable process --description 'Original' --prompt 'Original.'")
        .await?;

    let response = backend
        .exec_json(
            "agent update mutable.process process --description 'Updated' --prompt 'Updated.'",
        )
        .await?;

    assert!(
        matches!(
            response.data,
            Responses::Agent(AgentResponse::AgentUpdated(_))
        ),
        "expected AgentUpdated, got {response:#?}"
    );

    // Verify via show
    let show = backend.exec_json("agent show mutable.process").await?;

    match show.data {
        Responses::Agent(AgentResponse::AgentDetails(agent)) => {
            assert_eq!(agent.description.as_str(), "Updated");
        }
        other => panic!("expected AgentDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn remove_makes_it_unlisted<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    backend
        .exec_json("agent create temporary process --description 'Will be removed'")
        .await?;

    let response = backend.exec_json("agent remove temporary.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Agent(AgentResponse::AgentRemoved(_))
        ),
        "expected AgentRemoved, got {response:#?}"
    );

    let list = backend.exec_json("agent list").await?;

    assert!(
        matches!(list.data, Responses::Agent(AgentResponse::NoAgents)),
        "expected NoAgents after removal, got {list:#?}"
    );

    Ok(())
}

pub(crate) async fn name_includes_persona_suffix<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_persona(&mut backend).await?;

    backend
        .exec_json("agent create governor process --description 'Governor'")
        .await?;

    let response = backend.exec_json("agent show governor.process").await?;

    match response.data {
        Responses::Agent(AgentResponse::AgentDetails(agent)) => {
            assert_eq!(agent.name.as_str(), "governor.process");
        }
        other => panic!("expected AgentDetails, got {other:#?}"),
    }

    Ok(())
}
