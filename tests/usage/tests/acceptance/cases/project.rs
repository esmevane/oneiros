use oneiros_engine::*;
use oneiros_usage::*;

pub(crate) async fn init_creates_brain<B: Backend>() -> TestResult {
    let mut backend = B::start().await?;

    backend.exec("system init --name test --yes").await?;
    backend.start_service().await?;
    backend.exec("project init --yes").await?;

    // Verify brain exists by listing levels (requires a working brain context)
    let response = backend.exec("level list").await?;

    assert!(
        matches!(response.data, Responses::Level(LevelResponse::NoLevels)),
        "expected NoLevels from a fresh brain, got {response:#?}"
    );

    Ok(())
}
