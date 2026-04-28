//! Bootstrap workflow — from nothing to a functioning brain.
//!
//! This is the "hello world" of oneiros: initialize a system, create a
//! project, seed the vocabulary, emerge an agent, and dream them into
//! existence. If this works, the core lifecycle is sound.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn from_nothing_to_a_dreaming_agent() -> Result<(), Box<dyn core::error::Error>> {
    // Start fresh — initialize the system, create a project, seed the vocabulary
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    // The vocabulary is there — we can query it
    let client = app.client();
    match client
        .level()
        .list(&ListLevels::builder_v1().build().into())
        .await?
    {
        LevelResponse::Levels(LevelsResponse::V1(levels)) => {
            assert!(
                levels.items.len() >= 4,
                "seed should create at least 4 levels"
            );
        }
        other => panic!("expected Levels, got {other:?}"),
    }

    match client
        .persona()
        .list(&ListPersonas::builder_v1().build().into())
        .await?
    {
        PersonaResponse::Personas(PersonasResponse::V1(personas)) => {
            assert!(
                !personas.items.is_empty(),
                "seed should create at least one persona"
            );
        }
        other => panic!("expected Personas, got {other:?}"),
    }

    // Emerge an agent — this creates the agent and wakes them
    app.command("emerge thinker process").await?;

    // The agent exists
    match client
        .agent()
        .get(&GetAgent::V1(
            GetAgentV1::builder()
                .key(AgentName::new("thinker.process"))
                .build(),
        ))
        .await?
    {
        AgentResponse::AgentDetails(AgentDetailsResponse::V1(agent)) => {
            assert_eq!(agent.agent.name, AgentName::new("thinker.process"));
            assert_eq!(agent.agent.persona, PersonaName::new("process"));
        }
        other => panic!("expected AgentDetails, got {other:?}"),
    }

    // Dream them — assembles identity, vocabulary, and instructions
    let result = app.command("dream thinker.process").await?;
    let rendered = serde_json::to_value(result.response())?;
    let dream_text = rendered.to_string();

    assert!(
        dream_text.contains("thinker.process"),
        "dream should name the agent"
    );
    assert!(
        dream_text.contains("process"),
        "dream should reference their persona"
    );

    Ok(())
}

/// Project init should produce a "main" bookmark in the system DB.
///
/// Nothing implicit: if a resource exists, an event brought it into being
/// and a projection row reflects it. The default bookmark is no exception.
#[tokio::test]
async fn project_init_creates_main_bookmark() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    let client = app.client();

    match client
        .bookmark()
        .list(&ListBookmarks::builder_v1().build().into())
        .await?
    {
        BookmarkResponse::Bookmarks(bookmarks) => {
            assert_eq!(bookmarks.len(), 1, "exactly one bookmark after init");
            assert_eq!(
                bookmarks.items[0].name,
                BookmarkName::main(),
                "the bookmark should be named main"
            );
        }
        other => panic!("expected Bookmarks, got {other:?}"),
    }

    Ok(())
}
