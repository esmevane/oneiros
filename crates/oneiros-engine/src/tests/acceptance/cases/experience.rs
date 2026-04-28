use super::*;

/// Helper: init project + persona + sensation + agent for experience tests.
async fn with_agent_and_sensation<B: Backend>() -> Result<Harness<B>, Box<dyn core::error::Error>> {
    let harness = Harness::<B>::init_project().await?;
    harness
        .exec_json("persona set process --description 'Process agents'")
        .await?;
    harness
        .exec_json("sensation set caused --description 'One thought produced another'")
        .await?;
    harness
        .exec_json("agent create observer process --description 'An observing agent'")
        .await?;
    Ok(harness)
}

pub(crate) async fn create<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;

    let response = harness
        .exec_json("experience create observer.process caused 'A caused B'")
        .await?;

    assert!(
        matches!(
            response,
            Responses::Experience(ExperienceResponse::ExperienceCreated(_))
        ),
        "expected ExperienceCreated, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;

    let response = harness.exec_json("experience list").await?;

    assert!(
        matches!(
            response,
            Responses::Experience(ExperienceResponse::NoExperiences)
        ),
        "expected NoExperiences, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;

    harness
        .exec_json("experience create observer.process caused 'First experience'")
        .await?;
    harness
        .exec_json("experience create observer.process caused 'Second experience'")
        .await?;

    let response = harness.exec_json("experience list").await?;

    match response {
        Responses::Experience(ExperienceResponse::Experiences(ExperiencesResponse::V1(
            experiences,
        ))) => {
            assert_eq!(experiences.items.len(), 2);
        }
        other => panic!("expected Experiences, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_by_id<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;

    let create_response = harness
        .exec_json("experience create observer.process caused 'Show me this'")
        .await?;

    let id = match create_response {
        Responses::Experience(ExperienceResponse::ExperienceCreated(
            ExperienceCreatedResponse::V1(created),
        )) => created.experience.id,
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    let show_response = harness.exec_json(&format!("experience show {id}")).await?;

    match show_response {
        Responses::Experience(ExperienceResponse::ExperienceDetails(
            ExperienceDetailsResponse::V1(details),
        )) => {
            assert_eq!(details.experience.description.as_str(), "Show me this");
        }
        other => panic!("expected ExperienceDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_by_ref<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;

    let create_response = harness
        .exec_json("experience create observer.process caused 'Show me by ref'")
        .await?;

    let ref_token = match create_response {
        Responses::Experience(ExperienceResponse::ExperienceCreated(
            ExperienceCreatedResponse::V1(experience),
        )) => RefToken::new(Ref::experience(experience.experience.id)),
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    let show_response = harness
        .exec_json(&format!("experience show {ref_token}"))
        .await?;

    match show_response {
        Responses::Experience(ExperienceResponse::ExperienceDetails(
            ExperienceDetailsResponse::V1(experience),
        )) => {
            assert_eq!(experience.experience.description.as_str(), "Show me by ref");
        }
        other => panic!("expected ExperienceDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_by_wrong_kind_ref_errors<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;

    // Create a cognition so we have a cognition ref to misuse.
    harness
        .exec_json("texture set observation --description 'An observation'")
        .await?;
    let cognition_response = harness
        .exec_json("cognition add observer.process observation 'A noticed thing'")
        .await?;

    let cognition_ref = match cognition_response {
        Responses::Cognition(CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(
            cognition,
        ))) => RefToken::new(Ref::cognition(cognition.cognition.id)),
        other => panic!("expected CognitionAdded, got {other:#?}"),
    };

    let result = harness
        .exec_json(&format!("experience show {cognition_ref}"))
        .await;

    let Err(err) = result else {
        panic!("expected error for wrong-kind ref, got Ok");
    };
    let message = err.to_string();
    assert!(
        message.contains("experience") && message.contains("cognition"),
        "expected wrong-kind error naming both kinds, got: {message}"
    );

    Ok(())
}

pub(crate) async fn update_description<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;

    let create_response = harness
        .exec_json("experience create observer.process caused 'Original'")
        .await?;

    let id = match create_response {
        Responses::Experience(ExperienceResponse::ExperienceCreated(
            ExperienceCreatedResponse::V1(created),
        )) => created.experience.id,
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    let update_response = harness
        .exec_json(&format!(
            "experience update {id} --description 'Updated description'"
        ))
        .await?;

    assert!(
        matches!(
            update_response,
            Responses::Experience(ExperienceResponse::ExperienceUpdated(_))
        ),
        "expected ExperienceUpdated, got {update_response:?}"
    );

    let show_response = harness.exec_json(&format!("experience show {id}")).await?;

    match show_response {
        Responses::Experience(ExperienceResponse::ExperienceDetails(
            ExperienceDetailsResponse::V1(experience),
        )) => {
            assert_eq!(
                experience.experience.description.as_str(),
                "Updated description"
            );
        }
        other => panic!("expected ExperienceDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;

    let response = harness
        .exec_json("experience create observer.process caused 'Show this'")
        .await?;
    let id = match response {
        Responses::Experience(ExperienceResponse::ExperienceCreated(
            ExperienceCreatedResponse::V1(e),
        )) => e.experience.id.to_string(),
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    let prompt = harness
        .exec_prompt(&format!("experience show {id}"))
        .await?;

    assert!(
        !prompt.is_empty(),
        "experience show prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;
    harness
        .exec_json("experience create observer.process caused 'An experience'")
        .await?;

    let prompt = harness
        .exec_prompt("experience list --agent observer.process")
        .await?;

    assert!(
        !prompt.is_empty(),
        "experience list prompt should not be empty when experiences exist"
    );

    Ok(())
}

pub(crate) async fn update_prompt<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;

    let response = harness
        .exec_json("experience create observer.process caused 'Original'")
        .await?;
    let id = match response {
        Responses::Experience(ExperienceResponse::ExperienceCreated(
            ExperienceCreatedResponse::V1(e),
        )) => e.experience.id.to_string(),
        other => panic!("expected ExperienceCreated, got {other:#?}"),
    };

    let prompt = harness
        .exec_prompt(&format!("experience update {id} --description 'Updated'"))
        .await?;

    assert!(
        !prompt.is_empty(),
        "experience update prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn create_prompt_confirms_creation<B: Backend>() -> TestResult {
    let harness = with_agent_and_sensation::<B>().await?;

    let prompt = harness
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
