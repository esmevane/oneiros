use crate::*;
use crate::tests::acceptance::harness::*;

pub(crate) async fn set_creates<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness
        .exec_json("sensation set echoes --description 'Resonance between thoughts' --prompt 'Use when things rhyme.'")
        .await?;

    assert!(
        matches!(
            response,
            Responses::Sensation(SensationResponse::SensationSet(_))
        ),
        "expected SensationSet, got {response:#?}"
    );

    let show_response = harness.exec_json("sensation show echoes").await?;

    match show_response {
        Responses::Sensation(SensationResponse::SensationDetails(s)) => {
            assert_eq!(s.data.name.as_str(), "echoes");
            assert_eq!(s.data.description.as_str(), "Resonance between thoughts");
        }
        other => panic!("expected SensationDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn set_updates<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("sensation set draft --description 'Original' --prompt 'Original.'")
        .await?;

    harness
        .exec_json("sensation set draft --description 'Updated' --prompt 'Updated.'")
        .await?;

    let show_response = harness.exec_json("sensation show draft").await?;

    match show_response {
        Responses::Sensation(SensationResponse::SensationDetails(s)) => {
            assert_eq!(s.data.description.as_str(), "Updated");
        }
        other => panic!("expected SensationDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("sensation list").await?;

    assert!(
        matches!(
            response,
            Responses::Sensation(SensationResponse::NoSensations)
        ),
        "expected NoSensations, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("sensation set first --description 'First' --prompt 'First.'")
        .await?;

    harness
        .exec_json("sensation set second --description 'Second' --prompt 'Second.'")
        .await?;

    let response = harness.exec_json("sensation list").await?;

    match response {
        Responses::Sensation(SensationResponse::Sensations(list)) => {
            assert_eq!(list.len(), 2);
        }
        other => panic!("expected Sensations, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn remove<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("sensation set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let remove_response = harness.exec_json("sensation remove temporary").await?;

    assert!(
        matches!(
            remove_response,
            Responses::Sensation(SensationResponse::SensationRemoved(_))
        ),
        "expected SensationRemoved, got {remove_response:?}"
    );

    let list_response = harness.exec_json("sensation list").await?;

    assert!(
        matches!(
            list_response,
            Responses::Sensation(SensationResponse::NoSensations)
        ),
        "expected NoSensations after removal, got {list_response:?}"
    );

    Ok(())
}

pub(crate) async fn set_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness
        .exec_prompt(
            "sensation set test-sensation --description 'A test sensation' --prompt 'Test prompt.'",
        )
        .await?;

    assert!(
        !prompt.is_empty(),
        "sensation set prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json(
            "sensation set test-sensation --description 'A test sensation' --prompt 'Test prompt.'",
        )
        .await?;

    let prompt = harness.exec_prompt("sensation show test-sensation").await?;

    assert!(
        !prompt.is_empty(),
        "sensation show prompt should not be empty"
    );
    assert!(
        prompt.contains("test-sensation"),
        "sensation show prompt should contain the entry name"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json(
            "sensation set test-sensation --description 'A test sensation' --prompt 'Test prompt.'",
        )
        .await?;

    let prompt = harness.exec_prompt("sensation list").await?;

    assert!(
        !prompt.is_empty(),
        "sensation list prompt should not be empty when entries exist"
    );

    Ok(())
}

pub(crate) async fn remove_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("sensation set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let prompt = harness.exec_prompt("sensation remove temporary").await?;

    assert!(
        !prompt.is_empty(),
        "sensation remove prompt should not be empty"
    );

    Ok(())
}
