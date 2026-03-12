use oneiros_cli::{Init, InitProject};

use crate::*;

#[tokio::test]
async fn system_init_creates_tenant_and_actor() -> TestResult {
    let harness = TestHarness::new()?;

    Init::builder()
        .name("test")
        .yes(true)
        .build()
        .run(harness.context())
        .await?;

    let database = harness.context().database()?;

    assert!(database.tenant_exists()?);
    assert_eq!(database.event_count()?, 2);

    Ok(())
}

#[tokio::test]
async fn project_init_creates_brain() -> TestResult {
    let harness = TestHarness::new()?;

    Init::builder()
        .name("test")
        .yes(true)
        .build()
        .run(harness.context())
        .await?;

    let harness = harness.with_project("test-project").with_service().await?;

    InitProject::builder()
        .yes(true)
        .build()
        .run(harness.context())
        .await?;

    let token = harness.context().ticket_token()?;

    assert!(!token.0.is_empty());

    Ok(())
}
