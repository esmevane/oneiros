use super::*;

pub(crate) async fn init_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::setup_host().await?.start_service().await?;

    let prompt = harness.exec_prompt("project create --yes").await?;

    assert!(
        !prompt.is_empty(),
        "project create prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn init_creates_project<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("level list").await?;

    assert!(
        matches!(response, Responses::Level(LevelResponse::NoLevels)),
        "expected NoLevels from a fresh project, got {response:#?}"
    );

    Ok(())
}

pub(crate) async fn list_after_project_init<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("project list").await?;

    match response {
        Responses::Project(ProjectResponse::Listed(ProjectsResponse::V1(projects))) => {
            assert_eq!(
                projects.items.len(),
                1,
                "project create should create exactly one project"
            );
            assert_eq!(projects.items[0].name.as_str(), "test-project");
        }
        other => panic!("expected Project(Listed), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn get_by_name<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("project get test-project").await?;

    match response {
        Responses::Project(ProjectResponse::Found(ProjectFoundResponse::V1(found))) => {
            assert_eq!(found.project.name.as_str(), "test-project");
        }
        other => panic!("expected Project(Found), got {other:#?}"),
    }

    Ok(())
}

pub(crate) async fn list_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness.exec_prompt("project list").await?;

    assert!(
        !prompt.is_empty(),
        "project list prompt should not be empty"
    );
    assert!(
        prompt.contains("1 of"),
        "project list prompt should describe the project count, got: {prompt}"
    );

    Ok(())
}
