use super::*;

pub(crate) async fn core_prompt<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let prompt = harness.exec_prompt("seed core").await?;

    assert!(!prompt.is_empty(), "seed core prompt should not be empty");

    Ok(())
}

pub(crate) async fn core_creates_default_levels<B: Backend>() -> TestResult {
    let harness = Harness::<B>::init_project().await?;

    let response = harness.exec_json("seed core").await?;

    assert!(
        matches!(response, Responses::Seed(SeedResponse::SeedComplete)),
        "expected SeedComplete, got {response:#?}"
    );

    // Verify levels were created
    let list_response = harness.exec_json("level list").await?;

    match list_response {
        Responses::Level(LevelResponse::Levels(LevelsResponse::V1(levels))) => {
            // Core seed includes working, session, project, archival, core
            assert!(
                levels.items.len() >= 4,
                "expected at least 4 levels from core seed, got {}",
                levels.items.len()
            );
        }
        other => panic!("expected Levels, got {other:#?}"),
    }

    Ok(())
}
