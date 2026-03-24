use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn core_prompt<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;

    let prompt = backend.exec_prompt("seed core").await?;

    assert!(!prompt.is_empty(), "seed core prompt should not be empty");

    Ok(())
}

pub(crate) async fn core_creates_default_levels<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec_json("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec_json("project init --yes").await?;

    let response = backend.exec_json("seed core").await?;

    assert!(
        matches!(response.data, Responses::Seed(SeedResponse::SeedComplete)),
        "expected SeedComplete, got {response:#?}"
    );

    // Verify levels were created
    let list_response = backend.exec_json("level list").await?;

    match list_response.data {
        Responses::Level(LevelResponse::Levels(levels)) => {
            // Core seed includes working, session, project, archival, core
            assert!(
                levels.len() >= 4,
                "expected at least 4 levels from core seed, got {}",
                levels.len()
            );
        }
        other => panic!("expected Levels, got {other:#?}"),
    }

    Ok(())
}
