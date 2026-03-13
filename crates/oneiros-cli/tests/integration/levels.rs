use oneiros_cli::*;
use oneiros_model::*;

use crate::*;

#[tokio::test]
async fn set_level_creates_a_new_level() -> TestResult {
    let harness = TestHarness::new()?.bootstrap().await?;

    SetLevel::builder()
        .name("ephemeral")
        .description("Short-lived context")
        .prompt("Use for thoughts that won't outlast the session.")
        .build()
        .run(harness.context())
        .await?;

    let token = harness.token()?;
    let response = harness
        .client()
        .get_level(&token, &LevelName::new("ephemeral"))
        .await?;
    let level: Level = response.data()?;

    assert_eq!(level.name.as_str(), "ephemeral");
    assert_eq!(level.description.as_str(), "Short-lived context");
    assert_eq!(
        level.prompt.as_str(),
        "Use for thoughts that won't outlast the session."
    );

    Ok(())
}

#[tokio::test]
async fn list_levels_returns_empty_when_none_exist() -> TestResult {
    let harness = TestHarness::new()?.bootstrap().await?;

    let token = harness.token()?;
    let response = harness.client().list_levels(&token).await?;
    let levels: Vec<Level> = response.data()?;

    assert!(levels.is_empty());

    Ok(())
}

#[tokio::test]
async fn list_levels_includes_created_levels() -> TestResult {
    let harness = TestHarness::new()?.bootstrap().await?;

    SetLevel::builder()
        .name("session")
        .description("Current session context")
        .prompt("For the current session.")
        .build()
        .run(harness.context())
        .await?;

    SetLevel::builder()
        .name("project")
        .description("Project-lifetime knowledge")
        .prompt("For durable project knowledge.")
        .build()
        .run(harness.context())
        .await?;

    let token = harness.token()?;
    let response = harness.client().list_levels(&token).await?;
    let levels: Vec<Level> = response.data()?;

    assert_eq!(levels.len(), 2);

    Ok(())
}

#[tokio::test]
async fn set_level_updates_existing_level() -> TestResult {
    let harness = TestHarness::new()?.bootstrap().await?;

    SetLevel::builder()
        .name("working")
        .description("Original description")
        .prompt("Original prompt.")
        .build()
        .run(harness.context())
        .await?;

    SetLevel::builder()
        .name("working")
        .description("Updated description")
        .prompt("Updated prompt.")
        .build()
        .run(harness.context())
        .await?;

    let token = harness.token()?;
    let response = harness
        .client()
        .get_level(&token, &LevelName::new("working"))
        .await?;
    let level: Level = response.data()?;

    assert_eq!(level.description.as_str(), "Updated description");
    assert_eq!(level.prompt.as_str(), "Updated prompt.");

    Ok(())
}

#[tokio::test]
async fn remove_level_makes_it_unlisted() -> TestResult {
    let harness = TestHarness::new()?.bootstrap().await?;

    SetLevel::builder()
        .name("temporary")
        .description("Will be removed")
        .prompt("Temporary.")
        .build()
        .run(harness.context())
        .await?;

    RemoveLevel::builder()
        .name("temporary")
        .build()
        .run(harness.context())
        .await?;

    assert!(
        harness
            .client()
            .list_levels(&harness.token()?)
            .await?
            .data::<Vec<Level>>()?
            .is_empty()
    );

    Ok(())
}

#[tokio::test]
async fn seed_creates_default_levels() -> TestResult {
    let harness = TestHarness::new()?.bootstrap().await?;

    SeedOps::builder()
        .command(SeedCommands::Core)
        .build()
        .run(harness.context())
        .await?;

    let levels = harness
        .client()
        .list_levels(&harness.token()?)
        .await?
        .data::<Vec<Level>>()?;

    // Core seed includes working, session, project, archival
    assert!(levels.len() >= 4);

    Ok(())
}
