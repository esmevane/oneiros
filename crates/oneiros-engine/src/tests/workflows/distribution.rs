//! Distribution workflow — sharing continuity between projects.
//!
//! A project's continuity is shareable. Alice shares a bookmark, which
//! defines a view — what's visible, what's permitted. The share
//! produces an `oneiros://` link. Bob follows the link, creating a
//! local bookmark. He collects events into it, and merges when ready.
//!
//! These tests layer from foundation to full distribution:
//! 1. Imported material enters the dream (export/import, no network)
//! 2. Following a local project creates a bookmark from a view
//! 3. Collecting updates a followed bookmark with new events
//! 4. Views constrain what's visible to the follower
//! 5. Merging a followed bookmark integrates into the current timeline
//! 6. Provenance survives across follow chains

use crate::tests::harness::TestApp;
use crate::*;

// ── Foundation: import as proof of concept ──────────────────────

/// Imported material appears in the receiving agent's dream.
///
/// This is the portability test with a different question: not "did
/// the data survive?" but "does the dream incorporate foreign material?"
/// If this works, distribution is "just" the transport that automates
/// the import.
#[tokio::test]
async fn multi_source_dream() -> Result<(), Box<dyn core::error::Error>> {
    // ── Alice: a brain with rich cognitive history ─────────────

    let alice = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    alice.command("emerge thinker process").await?;

    alice
        .command(r#"cognition add thinker.process observation "The architecture is clean""#)
        .await?;
    alice
        .command(r#"cognition add thinker.process learning "Types enforce boundaries""#)
        .await?;
    alice
        .command(r#"memory add thinker.process core "I think in types""#)
        .await?;

    // Export Alice's brain
    let export_dir = tempfile::tempdir()?;
    alice
        .command(&format!(
            "project export --target {}",
            export_dir.path().display()
        ))
        .await?;

    let export_file = std::fs::read_dir(export_dir.path())?
        .filter_map(|e| e.ok())
        .find(|e| e.path().extension().is_some_and(|ext| ext == "jsonl"))
        .expect("export should produce a .jsonl file");

    // ── Bob: a separate brain with its own agent ──────────────

    let bob = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    bob.command("emerge listener process").await?;
    bob.command(r#"cognition add listener.process observation "Bob's own thought""#)
        .await?;

    // Bob's dream before import — only his own material
    let dream_before = match bob
        .client()
        .continuity()
        .dream(&AgentName::new("listener.process"))
        .await?
    {
        ContinuityResponse::Dreaming(ctx) => ctx,
        other => panic!("expected Dreaming, got {other:?}"),
    };

    let cognitions_before = dream_before.cognitions.len();
    let _memories_before = dream_before.memories.len();

    // ── Import Alice's brain into Bob's instance ──────────────

    bob.command(&format!("project import {}", export_file.path().display()))
        .await?;

    // ── Bob's dream now incorporates Alice's material ─────────

    // Alice's agent should exist on Bob's instance
    match bob
        .client()
        .agent()
        .get(&AgentName::new("thinker.process"))
        .await?
    {
        AgentResponse::AgentDetails(a) => {
            assert_eq!(a.data.name, AgentName::new("thinker.process"));
        }
        other => panic!("expected AgentDetails for Alice's agent, got {other:?}"),
    }

    // Alice's agent can dream on Bob's instance — full identity transfer
    let alice_dream_on_bob = match bob
        .client()
        .continuity()
        .dream(&AgentName::new("thinker.process"))
        .await?
    {
        ContinuityResponse::Dreaming(ctx) => ctx,
        other => panic!("expected Dreaming, got {other:?}"),
    };

    assert!(
        alice_dream_on_bob.cognitions.len() >= 2,
        "Alice's dream on Bob's instance should have her cognitions"
    );
    assert!(
        !alice_dream_on_bob.memories.is_empty(),
        "Alice's dream on Bob's instance should have her memories"
    );

    // Bob's own agent still dreams — and the brain is richer now
    let dream_after = match bob
        .client()
        .continuity()
        .dream(&AgentName::new("listener.process"))
        .await?
    {
        ContinuityResponse::Dreaming(ctx) => ctx,
        other => panic!("expected Dreaming, got {other:?}"),
    };

    // Bob's own cognitions are still there
    assert!(
        dream_after.cognitions.len() >= cognitions_before,
        "Bob should retain his own cognitions after import"
    );

    Ok(())
}

// ── Follow: a bookmark from a shared view ───────────────────────

/// Alice shares a bookmark. Bob follows the link, creating a local
/// bookmark. After collecting, the bookmark contains Alice's material.
/// Bob can switch to the bookmark and dream Alice's agent there.
#[tokio::test]
async fn follow_creates_bookmark() -> Result<(), Box<dyn core::error::Error>> {
    // ── Alice: a brain with content to share ──────────────────

    let alice = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    alice.command("emerge thinker process").await?;
    alice
        .command(r#"cognition add thinker.process observation "The sky is vast""#)
        .await?;
    alice
        .command(r#"memory add thinker.process core "I notice patterns""#)
        .await?;

    // Alice shares her main bookmark — gets back an oneiros:// link
    let link = alice.command("bookmark share main").await?;

    // ── Bob: follows Alice's link ─────────────────────────────

    let bob = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    bob.command("emerge listener process").await?;

    // Follow creates the bookmark — no events move yet
    bob.command(&format!("bookmark follow {} --name alice", link.prompt()))
        .await?;

    // The bookmark exists but is empty before collecting
    let result = bob.command("bookmark list").await?;
    let listing = result.prompt();
    assert!(listing.contains("alice"), "bookmark should appear in list");

    // Collect — events flow into the bookmark
    bob.command("bookmark collect alice").await?;

    // Switch to Alice's bookmark and verify her material is there
    bob.command("bookmark switch alice").await?;

    match bob
        .client()
        .agent()
        .get(&AgentName::new("thinker.process"))
        .await?
    {
        AgentResponse::AgentDetails(a) => {
            assert_eq!(a.data.name, AgentName::new("thinker.process"));
        }
        other => panic!("expected AgentDetails, got {other:?}"),
    }

    match bob
        .client()
        .continuity()
        .dream(&AgentName::new("thinker.process"))
        .await?
    {
        ContinuityResponse::Dreaming(ctx) => {
            assert!(
                !ctx.cognitions.is_empty(),
                "Alice's agent should have cognitions in the bookmark"
            );
            assert!(
                !ctx.memories.is_empty(),
                "Alice's agent should have memories in the bookmark"
            );
        }
        other => panic!("expected Dreaming, got {other:?}"),
    }

    Ok(())
}

// ── Scoped views: tickets filter what's visible ─────────────────

/// A scoped share limits what the follower sees. Alice has
/// observations and reflections. The share only offers observations.
/// Bob follows, collects, and only sees observations.
#[tokio::test]
#[ignore = "needs: bookmark slice (scoped fork creating a filtered bookmark)"]
async fn scoped_view_limits_visibility() -> Result<(), Box<dyn core::error::Error>> {
    let alice = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    alice.command("emerge thinker process").await?;
    alice
        .command(r#"cognition add thinker.process observation "This is visible""#)
        .await?;
    alice
        .command(r#"cognition add thinker.process reflection "This is private""#)
        .await?;

    // Alice shares main scoped to observations only
    let link = alice
        .command("bookmark share main --textures observation")
        .await?;

    let bob = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    bob.command(&format!(
        "bookmark follow {} --name alice-obs",
        link.prompt()
    ))
    .await?;
    bob.command("bookmark collect alice-obs").await?;
    bob.command("bookmark switch alice-obs").await?;

    // Only the observation should be present
    match bob
        .client()
        .cognition()
        .list(&ListCognitions {
            agent: Some(AgentName::new("thinker.process")),
            texture: None,
            filters: SearchFilters::default(),
        })
        .await?
    {
        CognitionResponse::Cognitions(cogs) => {
            assert_eq!(cogs.len(), 1, "only the observation should be visible");
            assert!(
                cogs.items[0].data.content.as_str().contains("visible"),
                "the visible cognition should be the observation"
            );
        }
        other => panic!("expected Cognitions, got {other:?}"),
    }

    Ok(())
}

// ── Collect: incremental updates ────────────────────────────────

/// Collecting a followed bookmark fetches only new events since the
/// last collection. Alice adds content over time. Each collect brings
/// only the delta.
#[tokio::test]
async fn collect_is_incremental() -> Result<(), Box<dyn core::error::Error>> {
    let alice = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    alice.command("emerge thinker process").await?;

    for i in 0..5 {
        alice
            .command(&format!(
                r#"cognition add thinker.process observation "Thought {i}""#
            ))
            .await?;
    }

    let link = alice.command("bookmark share main").await?;

    let bob = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    bob.command(&format!("bookmark follow {} --name alice", link.prompt()))
        .await?;

    // First collect — all 5 cognitions arrive
    bob.command("bookmark collect alice").await?;
    bob.command("bookmark switch alice").await?;

    match bob
        .client()
        .cognition()
        .list(&ListCognitions {
            agent: Some(AgentName::new("thinker.process")),
            texture: None,
            filters: SearchFilters::default(),
        })
        .await?
    {
        CognitionResponse::Cognitions(cogs) => {
            assert_eq!(cogs.len(), 5, "first collect should bring all 5");
        }
        other => panic!("expected Cognitions, got {other:?}"),
    }

    // Alice adds more
    for i in 5..8 {
        alice
            .command(&format!(
                r#"cognition add thinker.process observation "Thought {i}""#
            ))
            .await?;
    }

    // Second collect — only the 3 new ones
    bob.command("bookmark collect alice").await?;

    match bob
        .client()
        .cognition()
        .list(&ListCognitions {
            agent: Some(AgentName::new("thinker.process")),
            texture: None,
            filters: SearchFilters::default(),
        })
        .await?
    {
        CognitionResponse::Cognitions(cogs) => {
            assert_eq!(cogs.len(), 8, "second collect should bring total to 8");
        }
        other => panic!("expected Cognitions, got {other:?}"),
    }

    Ok(())
}

// ── Merge: integrating a followed bookmark ──────────────────────

/// After collecting, Bob merges the followed bookmark into his own
/// timeline. Alice's material becomes part of Bob's continuity.
/// His dream now incorporates both his own and Alice's content.
#[tokio::test]
async fn merge_integrates_followed_material() -> Result<(), Box<dyn core::error::Error>> {
    let alice = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    alice.command("emerge thinker process").await?;
    alice
        .command(r#"cognition add thinker.process observation "Alice's insight""#)
        .await?;
    alice
        .command(r#"memory add thinker.process core "Alice's knowledge""#)
        .await?;

    let link = alice.command("bookmark share main").await?;

    let bob = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    bob.command("emerge listener process").await?;
    bob.command(r#"cognition add listener.process observation "Bob's own thought""#)
        .await?;

    // Bob follows, collects, and merges Alice's bookmark into main
    bob.command(&format!("bookmark follow {} --name alice", link.prompt()))
        .await?;
    bob.command("bookmark collect alice").await?;
    bob.command("bookmark merge alice").await?;

    // Bob is on main — Alice's material is now part of his timeline
    // Alice's agent exists in Bob's main timeline
    match bob
        .client()
        .agent()
        .get(&AgentName::new("thinker.process"))
        .await?
    {
        AgentResponse::AgentDetails(a) => {
            assert_eq!(a.data.name, AgentName::new("thinker.process"));
        }
        other => panic!("expected AgentDetails, got {other:?}"),
    }

    // Bob's own agent still works — and can see the richer brain
    match bob
        .client()
        .continuity()
        .dream(&AgentName::new("listener.process"))
        .await?
    {
        ContinuityResponse::Dreaming(ctx) => {
            assert!(
                !ctx.cognitions.is_empty(),
                "Bob should still have his own cognitions"
            );
        }
        other => panic!("expected Dreaming, got {other:?}"),
    }

    // Alice's agent can also dream in Bob's main timeline
    match bob
        .client()
        .continuity()
        .dream(&AgentName::new("thinker.process"))
        .await?
    {
        ContinuityResponse::Dreaming(ctx) => {
            assert!(
                !ctx.cognitions.is_empty(),
                "Alice's agent should dream in Bob's main after merge"
            );
            assert!(
                !ctx.memories.is_empty(),
                "Alice's memories should be in Bob's main after merge"
            );
        }
        other => panic!("expected Dreaming, got {other:?}"),
    }

    Ok(())
}

// ── Provenance: lineage across follow chains ────────────────────

/// Events retain their origin across multiple follows.
/// Alice → Team → Bob: Bob can trace material back to Alice.
#[tokio::test]
async fn provenance_survives_follow_chain() -> Result<(), Box<dyn core::error::Error>> {
    // ── Three hosts ───────────────────────────────────────────

    let alice = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let team = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let bob = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    alice.command("emerge architect process").await?;
    alice
        .command(r#"cognition add architect.process assessment "Use event sourcing""#)
        .await?;

    // Alice shares → Team follows, collects, merges
    let alice_link = alice.command("bookmark share main").await?;
    team.command(&format!(
        "bookmark follow {} --name alice",
        alice_link.prompt()
    ))
    .await?;
    team.command("bookmark collect alice").await?;
    team.command("bookmark merge alice").await?;

    // Team shares → Bob follows, collects, merges
    let team_link = team.command("bookmark share main").await?;
    bob.command(&format!(
        "bookmark follow {} --name team",
        team_link.prompt()
    ))
    .await?;
    bob.command("bookmark collect team").await?;
    bob.command("bookmark merge team").await?;

    // Bob sees Alice's assessment — arrived through Team
    match bob
        .client()
        .cognition()
        .list(&ListCognitions {
            agent: Some(AgentName::new("architect.process")),
            texture: None,
            filters: SearchFilters::default(),
        })
        .await?
    {
        CognitionResponse::Cognitions(cogs) => {
            assert_eq!(cogs.len(), 1, "Alice's assessment should reach Bob");
            assert!(
                cogs.items[0]
                    .data
                    .content
                    .as_str()
                    .contains("event sourcing"),
                "the assessment content should be intact"
            );
        }
        other => panic!("expected Cognitions, got {other:?}"),
    }

    Ok(())
}

// ── Unfollow: releasing attention ───────────────────────────────

/// Unfollowing a bookmark stops future collects from reaching out.
/// The bookmark remains with whatever was last collected, but the
/// remote connection is severed.
#[tokio::test]
async fn unfollow_stops_collecting() -> Result<(), Box<dyn core::error::Error>> {
    let alice = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    alice.command("emerge thinker process").await?;
    alice
        .command(r#"cognition add thinker.process observation "Before unfollow""#)
        .await?;

    let link = alice.command("bookmark share main").await?;

    let bob = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    bob.command(&format!("bookmark follow {} --name alice", link.prompt()))
        .await?;
    bob.command("bookmark collect alice").await?;
    bob.command("bookmark switch alice").await?;

    // Verify initial content arrived
    match bob
        .client()
        .cognition()
        .list(&ListCognitions {
            agent: Some(AgentName::new("thinker.process")),
            texture: None,
            filters: SearchFilters::default(),
        })
        .await?
    {
        CognitionResponse::Cognitions(cogs) => {
            assert_eq!(cogs.len(), 1, "should have the initial cognition");
        }
        other => panic!("expected Cognitions, got {other:?}"),
    }

    // Bob unfollows
    bob.command("bookmark unfollow alice").await?;

    // Alice adds more content
    alice
        .command(r#"cognition add thinker.process observation "After unfollow""#)
        .await?;

    // Collecting after unfollow should not bring new events
    // (or should error — either way, no new content)
    let _ = bob.command("bookmark collect alice").await;

    match bob
        .client()
        .cognition()
        .list(&ListCognitions {
            agent: Some(AgentName::new("thinker.process")),
            texture: None,
            filters: SearchFilters::default(),
        })
        .await?
    {
        CognitionResponse::Cognitions(cogs) => {
            assert_eq!(
                cogs.len(),
                1,
                "should still have only the pre-unfollow cognition"
            );
        }
        other => panic!("expected Cognitions, got {other:?}"),
    }

    Ok(())
}
