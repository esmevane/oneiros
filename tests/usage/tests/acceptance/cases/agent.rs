use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: init project + create a persona so agents can reference it.
async fn with_persona<B: Backend>() -> Result<Harness<B>, Box<dyn core::error::Error>> {
    let harness = Harness::<B>::init_project().await?;
    harness
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    Ok(harness)
}

pub(crate) async fn create_with_persona<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;

    let response = harness
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
    let harness = with_persona::<B>().await?;

    harness
        .exec_json("agent create viewer process --description 'Views things'")
        .await?;

    let response = harness.exec_json("agent show viewer.process").await?;

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
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("agent list").await?;

    assert!(
        matches!(response.data, Responses::Agent(AgentResponse::NoAgents)),
        "expected NoAgents, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;

    harness
        .exec_json("agent create first process --description 'First'")
        .await?;
    harness
        .exec_json("agent create second process --description 'Second'")
        .await?;

    let response = harness.exec_json("agent list").await?;

    match response.data {
        Responses::Agent(AgentResponse::Agents(agents)) => {
            assert_eq!(agents.len(), 2);
        }
        other => panic!("expected Agents, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn update_changes_fields<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;

    harness
        .exec_json("agent create mutable process --description 'Original' --prompt 'Original.'")
        .await?;

    let response = harness
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

    let show = harness.exec_json("agent show mutable.process").await?;

    match show.data {
        Responses::Agent(AgentResponse::AgentDetails(agent)) => {
            assert_eq!(agent.description.as_str(), "Updated");
        }
        other => panic!("expected AgentDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn remove_makes_it_unlisted<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;

    harness
        .exec_json("agent create temporary process --description 'Will be removed'")
        .await?;

    let response = harness.exec_json("agent remove temporary.process").await?;

    assert!(
        matches!(
            response.data,
            Responses::Agent(AgentResponse::AgentRemoved(_))
        ),
        "expected AgentRemoved, got {response:#?}"
    );

    let list = harness.exec_json("agent list").await?;

    assert!(
        matches!(list.data, Responses::Agent(AgentResponse::NoAgents)),
        "expected NoAgents after removal, got {list:#?}"
    );

    Ok(())
}

pub(crate) async fn create_prompt<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;

    let prompt = harness
        .exec_prompt("agent create thinker process --description 'A thinking agent'")
        .await?;

    assert!(
        !prompt.is_empty(),
        "agent create prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    let prompt = harness.exec_prompt("agent show thinker.process").await?;

    assert!(!prompt.is_empty(), "agent show prompt should not be empty");
    assert!(
        prompt.contains("thinker.process"),
        "agent show prompt should contain the agent name"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    let prompt = harness.exec_prompt("agent list").await?;

    assert!(
        !prompt.is_empty(),
        "agent list prompt should not be empty when agents exist"
    );

    Ok(())
}

pub(crate) async fn update_prompt<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;
    harness
        .exec_json("agent create thinker process --description 'Original'")
        .await?;

    let prompt = harness
        .exec_prompt("agent update thinker.process process --description 'Updated'")
        .await?;

    assert!(
        !prompt.is_empty(),
        "agent update prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn remove_prompt<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;
    harness
        .exec_json("agent create thinker process --description 'Temporary'")
        .await?;

    let prompt = harness.exec_prompt("agent remove thinker.process").await?;

    assert!(
        !prompt.is_empty(),
        "agent remove prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn name_includes_persona_suffix<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;

    harness
        .exec_json("agent create governor process --description 'Governor'")
        .await?;

    let response = harness.exec_json("agent show governor.process").await?;

    match response.data {
        Responses::Agent(AgentResponse::AgentDetails(agent)) => {
            assert_eq!(agent.name.as_str(), "governor.process");
        }
        other => panic!("expected AgentDetails, got {other:#?}"),
    }

    Ok(())
}
