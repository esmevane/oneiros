use super::*;

pub(crate) async fn init_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::setup_system().await?.start_service().await?;

    let prompt = harness.exec_prompt("project init --yes").await?;

    assert!(
        !prompt.is_empty(),
        "project init prompt should not be empty"
    );

    Ok(())
}

pub(crate) async fn init_creates_brain<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    // Verify brain exists by listing levels (requires a working brain context)
    let response = harness.exec_json("level list").await?;

    assert!(
        matches!(response, Responses::Level(LevelResponse::NoLevels)),
        "expected NoLevels from a fresh brain, got {response:#?}"
    );

    Ok(())
}
