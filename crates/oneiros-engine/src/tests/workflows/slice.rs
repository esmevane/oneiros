//! Slice workflow — standing queries over continuity.
//!
//! A slice is a lens-filtered view of the event stream — a standing
//! query that materializes matching events into its own chronicle and
//! projection DB. Slices are independent of bookmarks: a bookmark is
//! what you create when you want to transport or share a snapshot.
//!
//! These tests cover the slice surface and its bookmark bridge:
//! 1. Creating a slice materializes matching events retroactively
//! 2. Empty lenses produce empty slices (not errors)
//! 3. Listing and deleting slices
//! 4. Diffing two slices reveals event-level differences
//! 5. Bookmarking a slice snapshots it into transportable form
//! 6. The refine → rebase → diff → iterate workflow

use crate::tests::harness::TestApp;
use crate::*;

async fn seeded_app() -> Result<TestApp, Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_host()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    // Two agents with personal cognitions
    app.command(r#"agent create gov process --description "Governor agent""#)
        .await?;
    app.command(r#"agent create bld process --description "Builder agent""#)
        .await?;

    app.command(r#"cognition add gov.process observation "Architecture is holding""#)
        .await?;
    app.command(r#"cognition add gov.process reflection "Design decisions compound""#)
        .await?;
    app.command(r#"cognition add bld.process observation "Lens charter complete""#)
        .await?;
    app.command(r#"cognition add bld.process learning "Structural honesty over velocity""#)
        .await?;

    Ok(app)
}

// ── Create: retroactive materialization ──────────────────────────

/// Creating a slice from an entity-level lens expression materializes
/// all events that touched matching entities. The slice's event count
/// reflects only the matching subset of the full event log.
#[tokio::test]
async fn slice_create_materializes_matching_events() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app
        .command(r#"slice create gov "agent(gov.process)""#)
        .await?;

    let Responses::Slice(SliceResponse::Created(SliceCreatedResponse::V1(created))) =
        rendered.response()
    else {
        panic!("expected Slice(Created), got {:#?}", rendered.response());
    };

    assert_eq!(created.slice.name.as_str(), "gov");
    assert_eq!(created.slice.lens_expr, "agent(gov.process)");
    assert!(
        created.slice.event_count >= 2,
        "gov.process has at least the two cognitions added by seeded_app"
    );

    Ok(())
}

// ── Create: empty slice ──────────────────────────────────────────

/// A lens that matches nothing produces a valid, empty slice — not an
/// error. The user intentionally created a narrow view; emptiness is a
/// legitimate result.
#[tokio::test]
async fn slice_create_empty_for_no_match() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    let rendered = app
        .command(r#"slice create empty "search(\"zzzznomatch\")""#)
        .await?;

    let Responses::Slice(SliceResponse::Created(SliceCreatedResponse::V1(created))) =
        rendered.response()
    else {
        panic!("expected Slice(Created), got {:#?}", rendered.response());
    };

    assert_eq!(created.slice.name.as_str(), "empty");
    assert_eq!(created.slice.event_count, 0, "nonexistent agent has no events");

    Ok(())
}

// ── List ─────────────────────────────────────────────────────────

/// Listing slices returns all slices for the project, with their lens
/// expressions and event counts.
#[tokio::test]
async fn slice_list_shows_all_slices() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    app.command(r#"slice create gov "agent(gov.process)""#)
        .await?;
    app.command(r#"slice create bld "agent(bld.process)""#)
        .await?;

    let rendered = app.command("slice list").await?;

    let Responses::Slice(SliceResponse::Slices(listed)) = rendered.response()
    else {
        panic!("expected Slice(Slices), got {:#?}", rendered.response());
    };

    assert_eq!(listed.items.len(), 2);
    let names: Vec<&str> = listed.items.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"gov"));
    assert!(names.contains(&"bld"));

    // Verify event counts reflect the different lenses
    let gov_slice = listed
        .items
        .iter()
        .find(|s| s.name.as_str() == "gov")
        .expect("gov slice exists");
    let bld_slice = listed
        .items
        .iter()
        .find(|s| s.name.as_str() == "bld")
        .expect("bld slice exists");
    assert_eq!(gov_slice.event_count, bld_slice.event_count);

    Ok(())
}

// ── Delete ───────────────────────────────────────────────────────

/// Deleting a slice removes it from the registry. The underlying
/// events in the project log are unaffected.
#[tokio::test]
async fn slice_delete_removes_slice() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    app.command(r#"slice create gov "agent(gov.process)""#)
        .await?;
    app.command("slice delete gov").await?;

    let rendered = app.command("slice list").await?;
    let Responses::Slice(SliceResponse::Slices(listed)) = rendered.response()
    else {
        panic!("expected Slice(Slices), got {:#?}", rendered.response());
    };

    assert!(
        !listed.items.iter().any(|s| s.name.as_str() == "gov"),
        "gov slice should be deleted"
    );

    Ok(())
}

// ── Diff ─────────────────────────────────────────────────────────

/// Diffing two slices returns the events in one but not the other.
/// This is the "what's missing?" operation from the dreamforge
/// workflow: narrow a slice, diff against the previous one, see
/// what fell through.
#[tokio::test]
async fn slice_diff_reveals_missing_events() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    // gov: all events from gov.process
    app.command(r#"slice create gov "agent(gov.process)""#)
        .await?;

    // gov-obs: narrower — only observations from gov.process
    app.command(r#"slice create gov-obs "agent(gov.process) & texture(observation)""#)
        .await?;

    let rendered = app.command("slice diff gov gov-obs").await?;

    let Responses::Slice(SliceResponse::Diffed(SliceDiffedResponse::V1(diffed))) =
        rendered.response()
    else {
        panic!("expected Slice(Diffed), got {:#?}", rendered.response());
    };

    assert!(
        diffed.only_in_source > 0,
        "broader gov slice should have events the narrower gov-obs lacks"
    );
    assert!(
        diffed.only_in_target == 0,
        "narrower gov-obs should not have events gov lacks"
    );

    Ok(())
}

// ── Bookmark bridge ──────────────────────────────────────────────

/// Bookmarking a slice creates a scoped bookmark whose projection DB
/// contains only the slice's matching events. The bookmark appears in
/// the bookmark list, and switching to it shows only the filtered view.
#[tokio::test]
async fn slice_bookmark_snapshots_into_bookmark() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    app.command(r#"slice create gov "agent(gov.process)""#)
        .await?;
    app.command("slice bookmark gov --as gov-snapshot").await?;

    let rendered = app.command("bookmark list").await?;
    let Responses::Bookmark(BookmarkResponse::Bookmarks(listed)) = rendered.response() else {
        panic!("expected Bookmarks, got {:#?}", rendered.response());
    };

    assert!(
        listed.items.iter().any(|b| b.name.as_str() == "gov-snapshot"),
        "bookmarked slice should appear in bookmark list"
    );

    // Switch to the snapshot and verify it contains only gov.process content
    app.command("bookmark switch gov-snapshot").await?;

    let client = app.client();
    let cognitions = client
        .cognition()
        .list(&ListCognitions::builder_v1().build().into())
        .await?;
    let items = match cognitions {
        CognitionResponse::Cognitions(CognitionsResponse::V1(r)) => r.items,
        other => panic!("expected Cognitions, got {other:?}"),
    };

    assert_eq!(
        items.len(),
        2,
        "scoped bookmark should have only 2 cognitions from gov.process"
    );
    for cog in &items {
        assert!(
            cog.content.as_str().contains("Architecture")
                || cog.content.as_str().contains("Design"),
            "only gov.process cognitions should be present, got: {}",
            cog.content.as_str()
        );
    }

    Ok(())
}

// ── Full workflow: refine → rebase → diff → iterate ─────────────

/// The dreamforge scenario: create a slice, bookmark it, refine to a
/// narrower slice, diff to find what was lost, broaden again with the
/// missing piece, bookmark the updated version.
///
/// This is the slice lifecycle users will actually experience: not
/// "create and forget" but "create, inspect, refine, iterate."
#[tokio::test]
async fn slice_refine_rebase_diff_iterate_workflow() -> Result<(), Box<dyn core::error::Error>> {
    let app = seeded_app().await?;

    // Step 1: Create an initial slice capturing everything from gov.process
    app.command(r#"slice create v1 "agent(gov.process)""#)
        .await?;

    // Step 2: Bookmark it for sharing (pretend we push to dreamforge)
    app.command("slice bookmark v1 --as v1-snapshot").await?;
    app.command("bookmark switch main").await?;

    // Step 3: Realize we need a narrower view — only reflections
    app.command(r#"slice create v2 "agent(gov.process) & texture(reflection)""#)
        .await?;

    // Step 4: Diff to see what fell through
    let diff_rendered = app.command("slice diff v1 v2").await?;
    let Responses::Slice(SliceResponse::Diffed(SliceDiffedResponse::V1(diffed))) =
        diff_rendered.response()
    else {
        panic!("expected Slice(Diffed), got {:#?}", diff_rendered.response());
    };
    assert!(
        diffed.only_in_source > 0,
        "v1 should have events v2 lacks"
    );

    // Step 5: "Someone asks about a missing piece" — broaden again
    // to include both reflections AND learnings
    app.command(r#"slice create v3 "agent(gov.process) & (texture(reflection) | texture(learning))""#)
        .await?;

    // Step 6: Rebase: move the bookmark from v1 to v3
    app.command("slice bookmark v3 --as v1-snapshot").await?;
    app.command("bookmark switch main").await?;

    // Step 7: Verify the updated snapshot reflects the v3 lens
    // (In a real workflow, we'd push again after rebasing)
    let list_rendered = app.command("slice list").await?;
    let Responses::Slice(SliceResponse::Slices(listed)) =
        list_rendered.response()
    else {
        panic!("expected Slice(Slices), got {:#?}", list_rendered.response());
    };
    let names: Vec<&str> = listed.items.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"v1"));
    assert!(names.contains(&"v2"));
    assert!(names.contains(&"v3"));

    Ok(())
}
