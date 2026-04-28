//! Versioning workflow — bookmark, switch, and merge the working canon.
//!
//! Bookmarks name timelines within a brain's canon. Creating a bookmark
//! forks the current state. Switching changes which timeline queries read
//! from. Merging combines two timelines into one.
//!
//! This test describes the destination: a brain where the read path is
//! bookmark-aware. It will fail until the query layer reads from the
//! active bookmark's canon instead of (or in addition to) SQLite.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn branch_switch_and_merge() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    app.command("emerge thinker process").await?;
    app.command("cognition add thinker.process observation 'thought on main'")
        .await?;

    // Confirm baseline: one agent, one cognition
    match client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(AgentName::new("thinker.process"))
                .build()
                .into(),
        )
        .await?
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
            assert_eq!(cogs.items.len(), 1)
        }
        other => panic!("expected 1 cognition on main, got {other:?}"),
    }

    app.command("bookmark create experiment").await?;

    // Add a cognition on the experiment branch
    app.command("cognition add thinker.process observation 'thought on experiment'")
        .await?;

    // Experiment should have both cognitions
    match client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(AgentName::new("thinker.process"))
                .build()
                .into(),
        )
        .await?
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
            assert_eq!(
                cogs.items.len(),
                2,
                "experiment branch should have 2 cognitions"
            )
        }
        other => panic!("expected 2 cognitions on experiment, got {other:?}"),
    }

    app.command("bookmark switch main").await?;

    // Main should still have only the original cognition
    match client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(AgentName::new("thinker.process"))
                .build()
                .into(),
        )
        .await?
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
            assert_eq!(
                cogs.items.len(),
                1,
                "main branch should still have 1 cognition"
            )
        }
        other => panic!("expected 1 cognition on main after switch, got {other:?}"),
    }

    app.command("bookmark switch experiment").await?;

    match client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(AgentName::new("thinker.process"))
                .build()
                .into(),
        )
        .await?
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => assert_eq!(
            cogs.items.len(),
            2,
            "experiment should still have 2 cognitions after round-trip"
        ),
        other => panic!("expected 2 cognitions on experiment after round-trip, got {other:?}"),
    }

    app.command("bookmark switch main").await?;
    app.command("bookmark merge experiment").await?;

    // Main should now have everything from both branches
    match client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(AgentName::new("thinker.process"))
                .build()
                .into(),
        )
        .await?
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
            assert_eq!(
                cogs.items.len(),
                2,
                "main should have 2 cognitions after merge"
            )
        }
        other => panic!("expected 2 cognitions on main after merge, got {other:?}"),
    }

    Ok(())
}

/// A fresh brain — one with no forks, no merges, nothing but project init —
/// should still show its `main` bookmark in `bookmark list`. This locks in
/// the "nothing implicit" rule: the default main bookmark exists via an
/// explicit `BookmarkCreated` event emitted at project init, not as an
/// implicit fallback from `CanonIndex::default`.
#[tokio::test]
async fn fresh_brain_lists_main_bookmark() -> Result<(), Box<dyn core::error::Error>> {
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
        BookmarkResponse::Bookmarks(listed) => {
            assert_eq!(
                listed.len(),
                1,
                "fresh brain should have exactly one bookmark entry"
            );
            assert_eq!(
                listed.items[0].name,
                BookmarkName::main(),
                "the sole bookmark should be `main`"
            );
        }
        other => panic!("expected Bookmarks response, got {other:?}"),
    }

    Ok(())
}
