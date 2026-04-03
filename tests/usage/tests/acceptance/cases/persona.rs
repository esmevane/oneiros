use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn set_creates<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness
        .exec_json("persona set process --description 'Process agents' --prompt 'Manages internal lifecycle.'")
        .await?;

    assert!(
        matches!(response, Responses::Persona(PersonaResponse::PersonaSet(_))),
        "expected PersonaSet, got {response:#?}"
    );

    let show_response = harness.exec_json("persona show process").await?;

    match show_response {
        Responses::Persona(PersonaResponse::PersonaDetails(p)) => {
            assert_eq!(p.data.name.as_str(), "process");
            assert_eq!(p.data.description.as_str(), "Process agents");
        }
        other => panic!("expected PersonaDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn set_updates<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("persona set draft --description 'Original' --prompt 'Original.'")
        .await?;

    harness
        .exec_json("persona set draft --description 'Updated' --prompt 'Updated.'")
        .await?;

    let show_response = harness.exec_json("persona show draft").await?;

    match show_response {
        Responses::Persona(PersonaResponse::PersonaDetails(p)) => {
            assert_eq!(p.data.description.as_str(), "Updated");
        }
        other => panic!("expected PersonaDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("persona list").await?;

    assert!(
        matches!(response, Responses::Persona(PersonaResponse::NoPersonas)),
        "expected NoPersonas, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("persona set first --description 'First' --prompt 'First.'")
        .await?;

    harness
        .exec_json("persona set second --description 'Second' --prompt 'Second.'")
        .await?;

    let response = harness.exec_json("persona list").await?;

    match response {
        Responses::Persona(PersonaResponse::Personas(list)) => {
            assert_eq!(list.len(), 2);
        }
        other => panic!("expected Personas, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn remove<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("persona set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let remove_response = harness.exec_json("persona remove temporary").await?;

    assert!(
        matches!(
            remove_response,
            Responses::Persona(PersonaResponse::PersonaRemoved(_))
        ),
        "expected PersonaRemoved, got {remove_response:?}"
    );

    let list_response = harness.exec_json("persona list").await?;

    assert!(
        matches!(
            list_response,
            Responses::Persona(PersonaResponse::NoPersonas)
        ),
        "expected NoPersonas after removal, got {list_response:?}"
    );

    Ok(())
}

pub(crate) async fn set_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness
        .exec_prompt(
            "persona set test-persona --description 'A test persona' --prompt 'Test prompt.'",
        )
        .await?;

    assert!(!prompt.is_empty(), "persona set prompt should not be empty");

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json(
            "persona set test-persona --description 'A test persona' --prompt 'Test prompt.'",
        )
        .await?;

    let prompt = harness.exec_prompt("persona show test-persona").await?;

    assert!(
        !prompt.is_empty(),
        "persona show prompt should not be empty"
    );
    assert!(
        prompt.contains("test-persona"),
        "persona show prompt should contain the entry name"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json(
            "persona set test-persona --description 'A test persona' --prompt 'Test prompt.'",
        )
        .await?;

    let prompt = harness.exec_prompt("persona list").await?;

    assert!(
        !prompt.is_empty(),
        "persona list prompt should not be empty when entries exist"
    );

    Ok(())
}

pub(crate) async fn remove_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("persona set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let prompt = harness.exec_prompt("persona remove temporary").await?;

    assert!(
        !prompt.is_empty(),
        "persona remove prompt should not be empty"
    );

    Ok(())
}
