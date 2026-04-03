//! Portability workflow — moving continuity between instances.
//!
//! A brain is a portable unit. An agent's identity, thoughts, memories,
//! experiences, and connections should survive export from one instance
//! and import into another. This is the distribution primitive: if
//! continuity survives the move, the identity survives.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn continuity_survives_export_import() -> Result<(), Box<dyn core::error::Error>> {
    // ── Instance A: build a rich cognitive history ───────────────

    let app_a = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client_a = app_a.client();

    // Emerge an agent
    client_a
        .continuity()
        .emerge(
            &EmergeAgent::builder()
                .name("thinker")
                .persona("process")
                .build(),
        )
        .await?;

    let agent = AgentName::new("thinker.process");

    // Populate cognitions
    app_a
        .command(r#"cognition add thinker.process observation "The architecture is clean""#)
        .await?;
    app_a
        .command(r#"cognition add thinker.process working "Exploring typed events""#)
        .await?;

    // Consolidate memories
    client_a
        .memory()
        .add(
            &AddMemory::builder()
                .agent(agent.clone())
                .level("core")
                .content("I think in types")
                .build(),
        )
        .await?;
    client_a
        .memory()
        .add(
            &AddMemory::builder()
                .agent(agent.clone())
                .level("project")
                .content("The engine consolidation is complete")
                .build(),
        )
        .await?;

    // Create an experience
    client_a
        .experience()
        .create(
            &CreateExperience::builder()
                .agent(agent.clone())
                .sensation("echoes")
                .description("Architecture and type safety resonate")
                .build(),
        )
        .await?;

    // Store an artifact
    client_a
        .storage()
        .upload(
            &UploadStorage::builder()
                .key("notes.md")
                .description("Session notes")
                .data(b"# Notes\nTypes are boundaries.".to_vec())
                .build(),
        )
        .await?;

    // Dream on instance A — capture what the dream looks like
    let dream_a = match client_a.continuity().dream(&agent).await? {
        ContinuityResponse::Dreaming(ctx) => ctx,
        other => panic!("expected Dreaming, got {other:?}"),
    };

    // Verify instance A has everything
    assert!(dream_a.cognitions.len() >= 2, "A should have cognitions");
    assert!(dream_a.memories.len() >= 2, "A should have memories");

    // ── Export from instance A ──────────────────────────────────

    let export_dir = tempfile::tempdir()?;
    app_a
        .command(&format!(
            "project export --target {}",
            export_dir.path().display()
        ))
        .await?;

    // Find the exported file
    let export_file = std::fs::read_dir(export_dir.path())?
        .filter_map(|e| e.ok())
        .find(|e| e.path().extension().is_some_and(|ext| ext == "jsonl"))
        .expect("export should produce a .jsonl file");

    // ── Instance B: a fresh brain ───────────────────────────────

    let app_b = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?;

    // Before import, the agent shouldn't exist
    let result = app_b.client().agent().get(&agent).await;
    assert!(result.is_err(), "agent should not exist before import");

    // ── Import into instance B ──────────────────────────────────

    app_b
        .command(&format!("project import {}", export_file.path().display()))
        .await?;

    let client_b = app_b.client();

    // ── Verify continuity survived ──────────────────────────────

    // The agent exists
    match client_b.agent().get(&agent).await? {
        AgentResponse::AgentDetails(a) => {
            assert_eq!(a.name, agent);
            assert_eq!(a.persona, PersonaName::new("process"));
        }
        other => panic!("expected AgentDetails, got {other:?}"),
    }

    // Their cognitions survived
    match client_b
        .cognition()
        .list(&ListCognitions {
            agent: Some(agent.clone()),
            texture: None,
            filters: SearchFilters::default(),
        })
        .await?
    {
        CognitionResponse::Cognitions(cogs) => {
            assert_eq!(
                cogs.len(),
                dream_a.cognitions.len(),
                "cognition count should match"
            );
        }
        other => panic!("expected Cognitions, got {other:?}"),
    }

    // Their memories survived
    match client_b
        .memory()
        .list(&ListMemories {
            agent: Some(agent.clone()),
            filters: SearchFilters::default(),
        })
        .await?
    {
        MemoryResponse::Memories(mems) => {
            assert_eq!(
                mems.len(),
                dream_a.memories.len(),
                "memory count should match"
            );
            let contents: Vec<&str> = mems.items.iter().map(|m| m.content.as_str()).collect();
            assert!(
                contents.contains(&"I think in types"),
                "core memory should survive"
            );
        }
        other => panic!("expected Memories, got {other:?}"),
    }

    // Their experiences survived
    match client_b
        .experience()
        .list(&ListExperiences {
            agent: Some(agent.clone()),
            filters: SearchFilters::default(),
        })
        .await?
    {
        ExperienceResponse::Experiences(exps) => {
            assert_eq!(exps.len(), 1, "experience should survive");
        }
        other => panic!("expected Experiences, got {other:?}"),
    }

    // Storage survived (including blob content)
    match client_b
        .storage()
        .show(&GetStorage::builder().key("notes.md").build())
        .await?
    {
        StorageResponse::StorageDetails(entry) => {
            assert_eq!(entry.key.as_str(), "notes.md");
            assert_eq!(entry.description.as_str(), "Session notes");
        }
        other => panic!("expected StorageDetails, got {other:?}"),
    }

    // The vocabulary survived
    match client_b.persona().get(&PersonaName::new("process")).await? {
        PersonaResponse::PersonaDetails(p) => {
            assert_eq!(p.name, PersonaName::new("process"));
        }
        other => panic!("expected PersonaDetails, got {other:?}"),
    }

    // Dream on instance B — the identity is intact
    let dream_b = match client_b.continuity().dream(&agent).await? {
        ContinuityResponse::Dreaming(ctx) => ctx,
        other => panic!("expected Dreaming, got {other:?}"),
    };

    assert_eq!(
        dream_a.cognitions.len(),
        dream_b.cognitions.len(),
        "dream should have same cognitions after import"
    );
    assert_eq!(
        dream_a.memories.len(),
        dream_b.memories.len(),
        "dream should have same memories after import"
    );
    assert_eq!(
        dream_b.agent.name, agent,
        "dream should reference the same agent"
    );

    Ok(())
}
