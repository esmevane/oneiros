//! Continuity workflow — the core cognitive loop.
//!
//! An agent wakes, thinks, remembers, connects ideas, dreams, and sleeps.
//! This is the heart of oneiros: the cycle from raw thought to
//! consolidated understanding, all assembled through dreaming.

use crate::tests::harness::TestApp;
use crate::*;

#[tokio::test]
async fn cognitive_session() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();
    let agent = AgentName::new("thinker.process");

    // An agent enters the world — via the continuity client
    client
        .continuity()
        .emerge(
            &EmergeAgent::builder_v1()
                .name("thinker")
                .persona("process")
                .build()
                .into(),
        )
        .await?;

    // They think — observations and working thoughts via CLI
    app.command(r#"cognition add thinker.process observation "The architecture is clean""#)
        .await?;
    app.command(r#"cognition add thinker.process working "Exploring typed events""#)
        .await?;
    app.command(r#"cognition add thinker.process reflection "Types enforce boundaries naturally""#)
        .await?;

    // Their thoughts are retrievable via client, filtered by texture
    match client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(agent.clone())
                .texture(TextureName::new("observation"))
                .build()
                .into(),
        )
        .await?
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(cogs)) => {
            assert_eq!(cogs.items.len(), 1)
        }
        other => panic!("expected Cognitions, got {other:?}"),
    }

    // They consolidate memories — via the typed client
    let core_memory = match client
        .memory()
        .add(
            &AddMemory::builder_v1()
                .agent(agent.clone())
                .level("core")
                .content("I think in types")
                .build()
                .into(),
        )
        .await?
    {
        MemoryResponse::MemoryAdded(MemoryAddedResponse::V1(added)) => {
            assert_eq!(added.memory.level.as_str(), "core");
            added.memory
        }
        other => panic!("expected MemoryAdded, got {other:?}"),
    };

    client
        .memory()
        .add(
            &AddMemory::builder_v1()
                .agent(agent.clone())
                .level("session")
                .content("Typed events enforce boundaries")
                .build()
                .into(),
        )
        .await?;

    match client
        .memory()
        .list(
            &ListMemories::builder_v1()
                .agent(agent.clone())
                .build()
                .into(),
        )
        .await?
    {
        MemoryResponse::Memories(MemoriesResponse::V1(mems)) => assert_eq!(mems.items.len(), 2),
        other => panic!("expected Memories, got {other:?}"),
    }

    // Mark a meaningful moment — via the typed client
    let experience = match client
        .experience()
        .create(
            &CreateExperience::builder_v1()
                .agent(agent.clone())
                .sensation("echoes")
                .description("Architecture and type safety resonate")
                .build()
                .into(),
        )
        .await?
    {
        ExperienceResponse::ExperienceCreated(ExperienceCreatedResponse::V1(created)) => {
            created.experience
        }
        other => panic!("expected ExperienceCreated, got {other:?}"),
    };

    match client
        .experience()
        .list(
            &ListExperiences::builder_v1()
                .agent(agent.clone())
                .build()
                .into(),
        )
        .await?
    {
        ExperienceResponse::Experiences(ExperiencesResponse::V1(exps)) => {
            assert_eq!(exps.items.len(), 1)
        }
        other => panic!("expected Experiences, got {other:?}"),
    }

    // Draw a connection — link memory to experience
    client
        .connection()
        .create(
            &CreateConnection::builder_v1()
                .from_ref(RefToken::new(Ref::memory(core_memory.id)))
                .to_ref(RefToken::new(Ref::experience(experience.id)))
                .nature("context")
                .build()
                .into(),
        )
        .await?;

    match client
        .connection()
        .list(&ListConnections::builder_v1().build().into())
        .await?
    {
        ConnectionResponse::Connections(ConnectionsResponse::V1(conns)) => {
            assert_eq!(conns.items.len(), 1)
        }
        other => panic!("expected Connections, got {other:?}"),
    }

    // Dream — via the continuity client
    match client.continuity().dream(&agent).await? {
        ContinuityResponse::Dreaming(DreamingResponse::V1(details)) => {
            let ctx = &details.context;
            assert!(ctx.cognitions.len() >= 3, "dream should include cognitions");
            assert!(ctx.memories.len() >= 2, "dream should include memories");
        }
        other => panic!("expected Dreaming, got {other:?}"),
    }

    // Check pressure after activity
    match client
        .pressure()
        .get(
            &GetPressure::builder_v1()
                .agent(agent.clone())
                .build()
                .into(),
        )
        .await?
    {
        PressureResponse::Readings(ReadingsResponse::V1(r)) => {
            assert!(
                !r.pressures.is_empty(),
                "should have pressure readings after activity"
            );
        }
        other => panic!("expected Readings, got {other:?}"),
    }

    // Introspect — via continuity client
    match client.continuity().introspect(&agent).await? {
        ContinuityResponse::Introspecting(_) => {}
        other => panic!("expected Introspecting, got {other:?}"),
    }

    // Sleep — end the session
    match client.continuity().sleep(&agent).await? {
        ContinuityResponse::Sleeping(_) => {}
        other => panic!("expected Sleeping, got {other:?}"),
    }

    Ok(())
}

#[tokio::test]
async fn listing_cognitions() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    // Create an agent and some cognitions
    app.command(r#"agent create thinker process --description "A thinking agent""#)
        .await?;

    app.command(r#"cognition add thinker.process observation "The sky is blue""#)
        .await?;
    app.command(r#"cognition add thinker.process working "Building something new""#)
        .await?;
    app.command(r#"cognition add thinker.process reflection "This approach feels right""#)
        .await?;

    // List should show actual items, not just a count
    let result = app
        .command("cognition list --agent thinker.process")
        .await?;

    let prompt = result.prompt();

    assert!(
        prompt.contains("3 of"),
        "expected '3 of' in prompt, got:\n{prompt}"
    );
    assert!(
        prompt.contains("The sky is blue"),
        "expected cognition content in listing, got:\n{prompt}"
    );
    assert!(
        prompt.contains("Building something new"),
        "expected cognition content in listing, got:\n{prompt}"
    );
    assert!(
        prompt.contains("This approach feels right"),
        "expected cognition content in listing, got:\n{prompt}"
    );
    assert!(
        prompt.contains("observation"),
        "expected texture label in listing, got:\n{prompt}"
    );
    assert!(
        prompt.contains("ref:"),
        "expected ref tokens in listing, got:\n{prompt}"
    );

    Ok(())
}

#[tokio::test]
async fn listing_cognitions_with_limit() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"agent create thinker process --description "A thinking agent""#)
        .await?;

    for i in 0..5 {
        app.command(&format!(
            r#"cognition add thinker.process observation "Thought number {i}""#
        ))
        .await?;
    }

    let result = app
        .command("cognition list --agent thinker.process --limit 2")
        .await?;

    let prompt = result.prompt();

    assert!(
        prompt.contains("2 of"),
        "expected '2 of' in prompt, got:\n{prompt}"
    );
    assert!(
        prompt.contains("of 5"),
        "expected 'of 5' total in prompt, got:\n{prompt}"
    );

    Ok(())
}

#[tokio::test]
async fn listing_cognitions_with_offset() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"agent create thinker process --description "A thinking agent""#)
        .await?;

    for i in 0..5 {
        app.command(&format!(
            r#"cognition add thinker.process observation "Thought {i}""#
        ))
        .await?;
    }

    let result = app
        .command("cognition list --agent thinker.process --offset 3")
        .await?;

    let prompt = result.prompt();

    assert!(
        prompt.contains("2 of"),
        "expected '2 of' in prompt, got:\n{prompt}"
    );
    assert!(
        prompt.contains("of 5"),
        "expected 'of 5' total in prompt, got:\n{prompt}"
    );

    Ok(())
}

#[tokio::test]
async fn listing_cognitions_empty() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"agent create thinker process --description "A thinking agent""#)
        .await?;

    let result = app
        .command("cognition list --agent thinker.process")
        .await?;
    let prompt = result.prompt();

    assert!(
        prompt.contains("No cognitions"),
        "expected 'No cognitions' for empty list, got:\n{prompt}"
    );

    Ok(())
}

#[tokio::test]
async fn listing_memories() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"agent create thinker process --description "A thinking agent""#)
        .await?;

    app.command(r#"memory add thinker.process session "Event sourcing is powerful""#)
        .await?;
    app.command(r#"memory add thinker.process project "Architecture settled""#)
        .await?;

    let result = app.command("memory list --agent thinker.process").await?;
    let prompt = result.prompt();

    assert!(
        prompt.contains("2 of"),
        "expected '2 of' in prompt, got:\n{prompt}"
    );
    assert!(
        prompt.contains("Event sourcing is powerful"),
        "expected memory content in listing, got:\n{prompt}"
    );
    assert!(
        prompt.contains("session"),
        "expected level label in listing, got:\n{prompt}"
    );
    assert!(
        prompt.contains("ref:"),
        "expected ref tokens in listing, got:\n{prompt}"
    );

    Ok(())
}

#[tokio::test]
async fn listing_experiences() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    app.command(r#"agent create thinker process --description "A thinking agent""#)
        .await?;

    app.command(r#"experience create thinker.process echoes "These insights rhyme""#)
        .await?;

    let result = app
        .command("experience list --agent thinker.process")
        .await?;
    let prompt = result.prompt();

    assert!(
        prompt.contains("1 of"),
        "expected '1 of' in prompt, got:\n{prompt}"
    );
    assert!(
        prompt.contains("These insights rhyme"),
        "expected experience description in listing, got:\n{prompt}"
    );
    assert!(
        prompt.contains("echoes"),
        "expected sensation label in listing, got:\n{prompt}"
    );

    Ok(())
}

#[tokio::test]
async fn agent_lifecycle() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();

    // Create directly (not emerge)
    app.command(r#"persona set custom --description "Custom persona" --prompt "You are custom""#)
        .await?;
    app.command(r#"agent create gov custom --description "The governor" --prompt "You govern""#)
        .await?;

    let agent = AgentName::new("gov.custom");

    // Verify via client
    match client
        .agent()
        .get(&GetAgent::V1(
            GetAgentV1::builder().key(agent.clone()).build(),
        ))
        .await?
    {
        AgentResponse::AgentDetails(AgentDetailsResponse::V1(a)) => {
            assert_eq!(a.agent.persona, PersonaName::new("custom"));
        }
        other => panic!("expected AgentDetails, got {other:?}"),
    }

    // Status — via continuity client
    match client.continuity().status().await? {
        ContinuityResponse::Status(StatusResponse::V1(details)) => {
            assert!(
                !details.table.agents.is_empty(),
                "should have agents in status"
            );
        }
        other => panic!("expected Status, got {other:?}"),
    }

    // Guidebook — via continuity client
    match client.continuity().guidebook(&agent).await? {
        ContinuityResponse::Guidebook(_) => {}
        other => panic!("expected Guidebook, got {other:?}"),
    }

    // Agent with nonexistent persona should fail
    let result = client
        .agent()
        .create(&CreateAgent::V1(
            CreateAgentV1::builder()
                .name("bad")
                .persona("nonexistent")
                .build(),
        ))
        .await;
    assert!(
        result.is_err(),
        "should reject agent with nonexistent persona"
    );

    // Duplicate agent should fail
    let result = client
        .agent()
        .create(&CreateAgent::V1(
            CreateAgentV1::builder()
                .name("gov")
                .persona("custom")
                .build(),
        ))
        .await;
    assert!(result.is_err(), "duplicate agent name should conflict");

    // Recede — via continuity client
    client.continuity().recede(&agent).await?;

    // Agent should be gone
    let result = client
        .agent()
        .get(&GetAgent::V1(
            GetAgentV1::builder().key(agent.clone()).build(),
        ))
        .await;
    assert!(result.is_err(), "receded agent should not be found");

    Ok(())
}

#[tokio::test]
async fn activity_status_shows_all_agents() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    // Create two agents with different activity levels
    app.command(r#"agent create alpha process --description "First agent""#)
        .await?;
    app.command(r#"agent create beta process --description "Second agent""#)
        .await?;

    // Alpha has cognitions and memories
    app.command(r#"cognition add alpha.process observation "Alpha sees things""#)
        .await?;
    app.command(r#"cognition add alpha.process working "Alpha is working""#)
        .await?;
    app.command(r#"memory add alpha.process session "Alpha remembers""#)
        .await?;

    // Beta has only one cognition
    app.command(r#"cognition add beta.process observation "Beta observes""#)
        .await?;

    // Status — no agent name required, shows all agents
    let result = app.command("continuity status").await?;
    let prompt = result.prompt();

    // Should contain both agent names
    assert!(
        prompt.contains("alpha.process"),
        "expected alpha.process in status, got:\n{prompt}"
    );
    assert!(
        prompt.contains("beta.process"),
        "expected beta.process in status, got:\n{prompt}"
    );

    // Should show counts — alpha has 2 cognitions, 1 memory
    // The table format shows Cog/Mem/Exp columns
    assert!(
        prompt.contains("Cog") || prompt.contains("cog"),
        "expected cognition column header in status, got:\n{prompt}"
    );

    Ok(())
}

/// The feedback scenario: list entities via the typed client, extract refs
/// from response meta, and use those refs to create connections — without
/// ever manually constructing a RefToken from a raw ID.
#[tokio::test]
async fn connect_via_listed_refs() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let client = app.client();
    let agent = AgentName::new("thinker.process");

    client
        .continuity()
        .emerge(
            &EmergeAgent::builder_v1()
                .name("thinker")
                .persona("process")
                .build()
                .into(),
        )
        .await?;

    // Add a cognition and a memory
    app.command(r#"cognition add thinker.process observation "A thought worth connecting""#)
        .await?;
    app.command(r#"memory add thinker.process session "A memory worth connecting""#)
        .await?;

    // List cognitions — derive ref from cognition id directly
    let cognition_ref = match client
        .cognition()
        .list(
            &ListCognitions::builder_v1()
                .agent(agent.clone())
                .build()
                .into(),
        )
        .await?
    {
        CognitionResponse::Cognitions(CognitionsResponse::V1(listed)) => {
            let first = &listed.items[0];
            RefToken::new(Ref::cognition(first.id))
        }
        other => panic!("expected Cognitions, got {other:?}"),
    };

    // List memories — derive ref from memory id directly
    let memory_ref = match client
        .memory()
        .list(
            &ListMemories::builder_v1()
                .agent(agent.clone())
                .build()
                .into(),
        )
        .await?
    {
        MemoryResponse::Memories(MemoriesResponse::V1(listed)) => {
            let first = &listed.items[0];
            RefToken::new(Ref::memory(first.id))
        }
        other => panic!("expected Memories, got {other:?}"),
    };

    // Connect using only the refs we extracted — no manual RefToken construction
    client
        .connection()
        .create(
            &CreateConnection::builder_v1()
                .from_ref(cognition_ref)
                .to_ref(memory_ref)
                .nature("context")
                .build()
                .into(),
        )
        .await?;

    // Verify the connection exists
    match client
        .connection()
        .list(&ListConnections::builder_v1().build().into())
        .await?
    {
        ConnectionResponse::Connections(ConnectionsResponse::V1(conns)) => {
            assert_eq!(conns.items.len(), 1)
        }
        other => panic!("expected Connections, got {other:?}"),
    }

    Ok(())
}
