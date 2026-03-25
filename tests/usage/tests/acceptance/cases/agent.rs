use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: init project + create a persona so agents can reference it.
async fn with_persona<B: Backend>() -> Result<Harness<B>, Box<dyn core::error::Error>> {
    let harness = Harness::<B>::init_project().await?;
    harness
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    harness
        .query("persona show process")
        .assert_json(expect!(Responses::Persona(
            PersonaResponse::PersonaDetails(_)
        )))
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

    harness
        .query("agent show viewer.process")
        .assert_json(expect!(
            Responses::Agent(AgentResponse::AgentDetails(agent))
                if agent.name.as_str() == "viewer.process"
                    && agent.persona.as_str() == "process"
        ))
        .await
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .query("agent list")
        .assert_json(expect!(Responses::Agent(AgentResponse::NoAgents)))
        .await
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;

    harness
        .exec_json("agent create first process --description 'First'")
        .await?;
    harness
        .exec_json("agent create second process --description 'Second'")
        .await?;

    harness
        .query("agent list")
        .assert_json(expect!(
            Responses::Agent(AgentResponse::Agents(agents)) if agents.len() == 2
        ))
        .await
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

    harness
        .query("agent show mutable.process")
        .assert_json(expect!(
            Responses::Agent(AgentResponse::AgentDetails(agent))
                if agent.description.as_str() == "Updated"
        ))
        .await
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

    harness
        .query("agent list")
        .assert_json(expect!(Responses::Agent(AgentResponse::NoAgents)))
        .await
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

    harness
        .query("agent show thinker.process")
        .assert_prompt(|prompt| {
            if prompt.is_empty() {
                return Err("agent show prompt should not be empty".into());
            }
            if !prompt.contains("thinker.process") {
                return Err("agent show prompt should contain the agent name".into());
            }
            Ok(())
        })
        .await
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = with_persona::<B>().await?;
    harness
        .exec_json("agent create thinker process --description 'A thinking agent'")
        .await?;

    harness
        .query("agent list")
        .assert_prompt(|prompt| {
            if prompt.is_empty() {
                Err("agent list prompt should not be empty when agents exist".into())
            } else {
                Ok(())
            }
        })
        .await
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

    harness
        .query("agent show governor.process")
        .assert_json(expect!(
            Responses::Agent(AgentResponse::AgentDetails(agent))
                if agent.name.as_str() == "governor.process"
        ))
        .await
}
