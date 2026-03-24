use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn set_creates_a_new_level<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;

    let response = backend
        .exec_json("level set ephemeral --description 'Short-lived context' --prompt 'Use for thoughts that will not outlast the session.'")
        .await?;

    assert!(
        matches!(response.data, Responses::Level(LevelResponse::LevelSet(_))),
        "expected LevelSet, got {response:#?}"
    );

    // Verify the level exists via show command
    let show_response = backend.exec_json("level show ephemeral").await?;

    match show_response.data {
        Responses::Level(LevelResponse::LevelDetails(level)) => {
            assert_eq!(level.name.as_str(), "ephemeral");
            assert_eq!(level.description.as_str(), "Short-lived context");
        }
        other => panic!("expected LevelDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn set_updates_existing_level<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;

    backend
        .exec_json(
            "level set working --description 'Original description' --prompt 'Original prompt.'",
        )
        .await?;

    backend
        .exec_json(
            "level set working --description 'Updated description' --prompt 'Updated prompt.'",
        )
        .await?;

    let show_response = backend.exec_json("level show working").await?;

    match show_response.data {
        Responses::Level(LevelResponse::LevelDetails(level)) => {
            assert_eq!(level.description.as_str(), "Updated description");
            assert_eq!(level.prompt.as_str(), "Updated prompt.");
        }
        other => panic!("expected LevelDetails, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_returns_empty_when_none_exist<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;

    let response = backend.exec_json("level list").await?;

    assert!(
        matches!(response.data, Responses::Level(LevelResponse::NoLevels)),
        "expected NoLevels, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_returns_created_levels<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;

    backend
        .exec_json("level set session --description 'Session context' --prompt 'For the session.'")
        .await?;

    backend
        .exec_json(
            "level set project --description 'Project knowledge' --prompt 'For the project.'",
        )
        .await?;

    let response = backend.exec_json("level list").await?;

    match response.data {
        Responses::Level(LevelResponse::Levels(levels)) => {
            assert_eq!(levels.len(), 2);
        }
        other => panic!("expected Levels, got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn set_prompt<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;

    let prompt = backend
        .exec_prompt("level set test-level --description 'A test level' --prompt 'Test prompt.'")
        .await?;

    assert!(!prompt.is_empty(), "level set prompt should not be empty");

    Ok(())
}

pub(crate) async fn show_prompt<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend
        .exec_json("level set test-level --description 'A test level' --prompt 'Test prompt.'")
        .await?;

    let prompt = backend.exec_prompt("level show test-level").await?;

    assert!(!prompt.is_empty(), "level show prompt should not be empty");
    assert!(
        prompt.contains("test-level"),
        "level show prompt should contain the entry name"
    );

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend
        .exec_json("level set test-level --description 'A test level' --prompt 'Test prompt.'")
        .await?;

    let prompt = backend.exec_prompt("level list").await?;

    assert!(
        !prompt.is_empty(),
        "level list prompt should not be empty when entries exist"
    );

    Ok(())
}

pub(crate) async fn remove_prompt<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;
    backend
        .exec_json("level set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let prompt = backend.exec_prompt("level remove temporary").await?;

    assert!(
        !prompt.is_empty(),
        "level remove prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn remove_makes_it_unlisted<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;

    backend
        .exec_json("level set temporary --description 'Will be removed' --prompt 'Temporary.'")
        .await?;

    let remove_response = backend.exec_json("level remove temporary").await?;

    assert!(
        matches!(
            remove_response.data,
            Responses::Level(LevelResponse::LevelRemoved(_))
        ),
        "expected LevelRemoved, got {remove_response:?}"
    );

    // Verify it's gone
    let list_response = backend.exec_json("level list").await?;

    assert!(
        matches!(
            list_response.data,
            Responses::Level(LevelResponse::NoLevels)
        ),
        "expected NoLevels after removal, got {list_response:?}"
    );

    Ok(())
}
