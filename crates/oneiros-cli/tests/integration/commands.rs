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

#[tokio::test]
async fn can_init_two_projects() -> TestResult {
    let harness = TestHarness::new()?;

    Init::builder()
        .name("test")
        .yes(true)
        .build()
        .run(harness.context())
        .await?;

    let harness = harness.with_project("project-a").with_service().await?;

    InitProject::builder()
        .yes(true)
        .build()
        .run(harness.context())
        .await?;

    let token_a = harness.context().ticket_token()?;
    assert!(!token_a.0.is_empty());

    // Now init a second project on the same service
    let harness = harness.with_project("project-b");

    InitProject::builder()
        .yes(true)
        .build()
        .run(harness.context())
        .await?;

    let token_b = harness.context().ticket_token()?;
    assert!(!token_b.0.is_empty());
    assert_ne!(
        token_a.0, token_b.0,
        "each project should get its own token"
    );

    Ok(())
}
