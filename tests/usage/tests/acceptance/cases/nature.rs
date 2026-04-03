use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn set_creates<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness
        .exec_json("nature set context --description 'Provides background' --prompt 'Use when one thing frames another.'")
        .await?;

    assert!(
        matches!(response, Responses::Nature(NatureResponse::NatureSet(_))),
        "expected NatureSet, got {response:#?}"
    );

    let show_response = harness.exec_json("nature show context").await?;

    match show_response {
        Responses::Nature(NatureResponse::NatureDetails(n)) => {
            assert_eq!(n.data.name.as_str(), "context");
            assert_eq!(n.data.description.as_str(), "Provides background");
        }
        other => panic!("expected NatureDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn set_updates<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("nature set draft --description 'Original' --prompt 'Original.'")
        .await?;

    harness
        .exec_json("nature set draft --description 'Updated' --prompt 'Updated.'")
        .await?;

    let show_response = harness.exec_json("nature show draft").await?;

    match show_response {
        Responses::Nature(NatureResponse::NatureDetails(n)) => {
            assert_eq!(n.data.description.as_str(), "Updated");
        }
        other => panic!("expected NatureDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("nature list").await?;

    assert!(
        matches!(response, Responses::Nature(NatureResponse::NoNatures)),
        "expected NoNatures, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("nature set first --description 'First' --prompt 'First.'")
        .await?;

    harness
        .exec_json("nature set second --description 'Second' --prompt 'Second.'")
        .await?;

    let response = harness.exec_json("nature list").await?;

    match response {
        Responses::Nature(NatureResponse::Natures(list)) => {
            assert_eq!(list.len(), 2);
        }
        other => panic!("expected Natures, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn remove<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("nature set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let remove_response = harness.exec_json("nature remove temporary").await?;

    assert!(
        matches!(
            remove_response,
            Responses::Nature(NatureResponse::NatureRemoved(_))
        ),
        "expected NatureRemoved, got {remove_response:?}"
    );

    let list_response = harness.exec_json("nature list").await?;

    assert!(
        matches!(list_response, Responses::Nature(NatureResponse::NoNatures)),
        "expected NoNatures after removal, got {list_response:?}"
    );

    Ok(())
}

pub(crate) async fn set_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness
        .exec_prompt("nature set test-nature --description 'A test nature' --prompt 'Test prompt.'")
        .await?;

    assert!(!prompt.is_empty(), "nature set prompt should not be empty");

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("nature set test-nature --description 'A test nature' --prompt 'Test prompt.'")
        .await?;

    let prompt = harness.exec_prompt("nature show test-nature").await?;

    assert!(!prompt.is_empty(), "nature show prompt should not be empty");
    assert!(
        prompt.contains("test-nature"),
        "nature show prompt should contain the entry name"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("nature set test-nature --description 'A test nature' --prompt 'Test prompt.'")
        .await?;

    let prompt = harness.exec_prompt("nature list").await?;

    assert!(
        !prompt.is_empty(),
        "nature list prompt should not be empty when entries exist"
    );

    Ok(())
}

pub(crate) async fn remove_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("nature set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let prompt = harness.exec_prompt("nature remove temporary").await?;

    assert!(
        !prompt.is_empty(),
        "nature remove prompt should not be empty"
    );

    Ok(())
}
