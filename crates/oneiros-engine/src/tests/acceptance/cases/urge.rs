use crate::*;
use crate::tests::acceptance::harness::*;

pub(crate) async fn set_creates<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness
        .exec_json("urge set introspect --description 'The pull to look inward' --prompt 'Pause and examine.'")
        .await?;

    assert!(
        matches!(response, Responses::Urge(UrgeResponse::UrgeSet(_))),
        "expected UrgeSet, got {response:#?}"
    );

    let show_response = harness.exec_json("urge show introspect").await?;

    match show_response {
        Responses::Urge(UrgeResponse::UrgeDetails(u)) => {
            assert_eq!(u.name.as_str(), "introspect");
            assert_eq!(u.description.as_str(), "The pull to look inward");
        }
        other => panic!("expected UrgeDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn set_updates<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("urge set draft --description 'Original' --prompt 'Original.'")
        .await?;

    harness
        .exec_json("urge set draft --description 'Updated' --prompt 'Updated.'")
        .await?;

    let show_response = harness.exec_json("urge show draft").await?;

    match show_response {
        Responses::Urge(UrgeResponse::UrgeDetails(u)) => {
            assert_eq!(u.description.as_str(), "Updated");
        }
        other => panic!("expected UrgeDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_empty<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("urge list").await?;

    assert!(
        matches!(response, Responses::Urge(UrgeResponse::NoUrges)),
        "expected NoUrges, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_populated<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("urge set first --description 'First' --prompt 'First.'")
        .await?;

    harness
        .exec_json("urge set second --description 'Second' --prompt 'Second.'")
        .await?;

    let response = harness.exec_json("urge list").await?;

    match response {
        Responses::Urge(UrgeResponse::Urges(list)) => {
            assert_eq!(list.len(), 2);
        }
        other => panic!("expected Urges, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn remove<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("urge set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let remove_response = harness.exec_json("urge remove temporary").await?;

    assert!(
        matches!(
            remove_response,
            Responses::Urge(UrgeResponse::UrgeRemoved(_))
        ),
        "expected UrgeRemoved, got {remove_response:?}"
    );

    let list_response = harness.exec_json("urge list").await?;

    assert!(
        matches!(list_response, Responses::Urge(UrgeResponse::NoUrges)),
        "expected NoUrges after removal, got {list_response:?}"
    );

    Ok(())
}

pub(crate) async fn set_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness
        .exec_prompt("urge set test-urge --description 'A test urge' --prompt 'Test prompt.'")
        .await?;

    assert!(!prompt.is_empty(), "urge set prompt should not be empty");

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("urge set test-urge --description 'A test urge' --prompt 'Test prompt.'")
        .await?;

    let prompt = harness.exec_prompt("urge show test-urge").await?;

    assert!(!prompt.is_empty(), "urge show prompt should not be empty");
    assert!(
        prompt.contains("test-urge"),
        "urge show prompt should contain the entry name"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("urge set test-urge --description 'A test urge' --prompt 'Test prompt.'")
        .await?;

    let prompt = harness.exec_prompt("urge list").await?;

    assert!(
        !prompt.is_empty(),
        "urge list prompt should not be empty when entries exist"
    );

    Ok(())
}

pub(crate) async fn remove_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    harness
        .exec_json("urge set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let prompt = harness.exec_prompt("urge remove temporary").await?;

    assert!(!prompt.is_empty(), "urge remove prompt should not be empty");

    Ok(())
}
