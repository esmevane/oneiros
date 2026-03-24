use oneiros_engine::*;
use oneiros_usage::*;

/// Helper: bootstrap with persona + agent + sensation for experiences.
async fn setup_with_agent_and_sensation<B: Backend>(backend: &mut B) -> TestResult {
    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    backend
        .exec_json("sensation set caused --description 'One thought produced another'")
        .await?;
    backend
        .exec_json("agent create observer process --description 'An observing agent'")
        .await?;
    Ok(())
}

pub(crate) async fn create<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let response = backend
        .exec_json("experience create observer.process caused 'A caused B'")
        .await?;

    assert!(
        matches!(
            response.data,
            Responses::Experience(ExperienceResponse::ExperienceCreated(_))
        ),
        "expected ExperienceCreated, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let response = backend.exec_json("experience list").await?;

    assert!(
        matches!(
            response.data,
            Responses::Experience(ExperienceResponse::NoExperiences)
        ),
        "expected NoExperiences, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    backend
        .exec_json("experience create observer.process caused 'First experience'")
        .await?;
    backend
        .exec_json("experience create observer.process caused 'Second experience'")
        .await?;

    let response = backend.exec_json("experience list").await?;

    match response.data {
        Responses::Experience(ExperienceResponse::Experiences(experiences)) => {
            assert_eq!(experiences.len(), 2);
        }
        other => panic!("expected Experiences, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let create_response = backend
        .exec_json("experience create observer.process caused 'Show me this'")
        .await?;

    let id = match create_response.data {
        Responses::Experience(ExperienceResponse::ExperienceCreated(experience)) => experience.id,
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    let show_cmd = format!("experience show {id}");
    let show_response = backend.exec_json(&show_cmd).await?;

    match show_response.data {
        Responses::Experience(ExperienceResponse::ExperienceDetails(experience)) => {
            assert_eq!(experience.description.as_str(), "Show me this");
        }
        other => panic!("expected ExperienceDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn update_description<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let create_response = backend
        .exec_json("experience create observer.process caused 'Original'")
        .await?;

    let id = match create_response.data {
        Responses::Experience(ExperienceResponse::ExperienceCreated(experience)) => experience.id,
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    let update_cmd = format!("experience update {id} --description 'Updated description'");
    let update_response = backend.exec_json(&update_cmd).await?;

    assert!(
        matches!(
            update_response.data,
            Responses::Experience(ExperienceResponse::ExperienceUpdated(_))
        ),
        "expected ExperienceUpdated, got {update_response:?}"
    );

    // Verify via show
    let show_cmd = format!("experience show {id}");
    let show_response = backend.exec_json(&show_cmd).await?;

    match show_response.data {
        Responses::Experience(ExperienceResponse::ExperienceDetails(experience)) => {
            assert_eq!(experience.description.as_str(), "Updated description");
        }
        other => panic!("expected ExperienceDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let response = backend
        .exec_json("experience create observer.process caused 'Show this'")
        .await?;
    let id = match response.data {
        Responses::Experience(ExperienceResponse::ExperienceCreated(e)) => e.id.to_string(),
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    let prompt = backend
        .exec_prompt(&format!("experience show {id}"))
        .await?;

    assert!(
        !prompt.is_empty(),
        "experience show prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;
    backend
        .exec_json("experience create observer.process caused 'An experience'")
        .await?;

    let prompt = backend
        .exec_prompt("experience list --agent observer.process")
        .await?;

    assert!(
        !prompt.is_empty(),
        "experience list prompt should not be empty when experiences exist"
    );

    Ok(())
}

pub(crate) async fn update_prompt<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let response = backend
        .exec_json("experience create observer.process caused 'Original'")
        .await?;
    let id = match response.data {
        Responses::Experience(ExperienceResponse::ExperienceCreated(e)) => e.id.to_string(),
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    let prompt = backend
        .exec_prompt(&format!("experience update {id} --description 'Updated'"))
        .await?;

    assert!(
        !prompt.is_empty(),
        "experience update prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn create_prompt_confirms_creation<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;
    setup_with_agent_and_sensation(&mut backend).await?;

    let prompt = backend
        .exec_prompt("experience create observer.process caused 'A prompted experience'")
        .await?;

    assert!(
        !prompt.is_empty(),
        "experience create prompt should not be empty — confirm what was recorded"
    );
    assert!(
        prompt.contains("ref:"),
        "experience create prompt should contain a ref token for the created experience"
    );

    Ok(())
}
