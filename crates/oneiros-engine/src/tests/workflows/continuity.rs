//! Continuity workflow — the core cognitive loop.
//!
//! An agent wakes, thinks, remembers, connects ideas, dreams, and sleeps.
//! This is the heart of oneiros: the cycle from raw thought to
//! consolidated understanding, all assembled through dreaming.

use futures::StreamExt;

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
            &EmergeAgent::builder()
                .name("thinker")
                .persona("process")
                .build(),
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
        .list(&ListCognitions {
            agent: Some(agent.clone()),
            texture: Some(TextureName::new("observation")),
        })
        .await?
    {
        CognitionResponse::Cognitions(cogs) => assert_eq!(cogs.len(), 1),
        other => panic!("expected Cognitions, got {other:?}"),
    }

    // They consolidate memories — via the typed client
    let core_memory = match client
        .memory()
        .add(
            &AddMemory::builder()
                .agent(agent.clone())
                .level("core")
                .content("I think in types")
                .build(),
        )
        .await?
    {
        MemoryResponse::MemoryAdded(m) => {
            assert_eq!(m.level.as_str(), "core");
            m
        }
        other => panic!("expected MemoryAdded, got {other:?}"),
    };

    client
        .memory()
        .add(
            &AddMemory::builder()
                .agent(agent.clone())
                .level("session")
                .content("Typed events enforce boundaries")
                .build(),
        )
        .await?;

    match client
        .memory()
        .list(&ListMemories {
            agent: Some(agent.clone()),
        })
        .await?
    {
        MemoryResponse::Memories(mems) => assert_eq!(mems.len(), 2),
        other => panic!("expected Memories, got {other:?}"),
    }

    // Mark a meaningful moment — via the typed client
    let experience = match client
        .experience()
        .create(
            &CreateExperience::builder()
                .agent(agent.clone())
                .sensation("echoes")
                .description("Architecture and type safety resonate")
                .build(),
        )
        .await?
    {
        ExperienceResponse::ExperienceCreated(e) => e,
        other => panic!("expected ExperienceCreated, got {other:?}"),
    };

    match client
        .experience()
        .list(&ListExperiences {
            agent: Some(agent.clone()),
        })
        .await?
    {
        ExperienceResponse::Experiences(exps) => assert_eq!(exps.len(), 1),
        other => panic!("expected Experiences, got {other:?}"),
    }

    // Draw a connection — link memory to experience
    client
        .connection()
        .create(
            &CreateConnection::builder()
                .from_ref(RefToken::new(Ref::memory(core_memory.id)))
                .to_ref(RefToken::new(Ref::experience(experience.id)))
                .nature("context")
                .build(),
        )
        .await?;

    match client
        .connection()
        .list(&ListConnections { entity: None })
        .await?
    {
        ConnectionResponse::Connections(conns) => assert_eq!(conns.len(), 1),
        other => panic!("expected Connections, got {other:?}"),
    }

    // Dream — via the continuity client
    match client.continuity().dream(&agent).await? {
        ContinuityResponse::Dreaming(ctx) => {
            assert!(ctx.cognitions.len() >= 3, "dream should include cognitions");
            assert!(ctx.memories.len() >= 2, "dream should include memories");
        }
        other => panic!("expected Dreaming, got {other:?}"),
    }

    // Check pressure after activity
    match client
        .pressure()
        .get(&GetPressure::builder().agent(agent.clone()).build())
        .await?
    {
        PressureResponse::Readings(r) => {
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
    match client.agent().get(&agent).await? {
        AgentResponse::AgentDetails(a) => {
            assert_eq!(a.persona, PersonaName::new("custom"));
        }
        other => panic!("expected AgentDetails, got {other:?}"),
    }

    // Status — via continuity client
    match client.continuity().status(&agent).await? {
        ContinuityResponse::Status(_) => {}
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
        .create(
            &CreateAgent::builder()
                .name("bad")
                .persona("nonexistent")
                .build(),
        )
        .await;
    assert!(
        result.is_err(),
        "should reject agent with nonexistent persona"
    );

    // Duplicate agent should fail
    let result = client
        .agent()
        .create(&CreateAgent::builder().name("gov").persona("custom").build())
        .await;
    assert!(result.is_err(), "duplicate agent name should conflict");

    // Recede — via continuity client
    client.continuity().recede(&agent).await?;

    // Agent should be gone
    let result = client.agent().get(&agent).await;
    assert!(result.is_err(), "receded agent should not be found");

    Ok(())
}

#[tokio::test]
async fn activity_stream_observes_events() -> Result<(), Box<dyn core::error::Error>> {
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let token = app.token().expect("should have a token after init");
    let sse_url = format!("{}/activity", app.base_url());

    // Subscribe to the SSE activity stream in a background task
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<StoredEvent>(64);

    let sse_handle = tokio::spawn(async move {
        let http = reqwest::Client::new();
        let resp = http
            .get(&sse_url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Accept", "text/event-stream")
            .send()
            .await
            .unwrap();

        let mut stream = resp.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let bytes = chunk.unwrap();
            let text = String::from_utf8_lossy(&bytes);
            buffer.push_str(&text);

            // Parse SSE events from the buffer
            while let Some(pos) = buffer.find("\n\n") {
                let event_block = buffer[..pos].to_string();
                buffer = buffer[pos + 2..].to_string();

                let data_line = event_block
                    .lines()
                    .find(|l| l.starts_with("data:"))
                    .and_then(|l| {
                        let trimmed = l.strip_prefix("data:").unwrap().trim();
                        if trimmed.is_empty() {
                            None
                        } else {
                            Some(trimmed.to_string())
                        }
                    });

                if let Some(event) =
                    data_line.and_then(|d| serde_json::from_str::<StoredEvent>(&d).ok())
                    && event_tx.send(event).await.is_err()
                {
                    return;
                }
            }
        }
    });

    // Give the SSE connection a moment to establish
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Perform operations that should emit events
    app.command("agent create thinker process").await?;
    app.command(r#"cognition add thinker.process observation "A thought""#)
        .await?;
    app.command(r#"memory add thinker.process session "A memory""#)
        .await?;

    // Give events time to propagate through SSE
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    sse_handle.abort();

    // Collect received events
    let mut events = Vec::new();
    while let Ok(event) = event_rx.try_recv() {
        events.push(event);
    }

    assert!(!events.is_empty(), "SSE stream should have received events");

    let event_types: Vec<&str> = events.iter().map(|e| e.data.event_type()).collect();

    assert!(
        event_types.contains(&"agent-created"),
        "stream should contain agent-created, got: {event_types:?}"
    );
    assert!(
        event_types.contains(&"cognition-added"),
        "stream should contain cognition-added, got: {event_types:?}"
    );
    assert!(
        event_types.contains(&"memory-added"),
        "stream should contain memory-added, got: {event_types:?}"
    );

    // Events should be in sequence order
    for window in events.windows(2) {
        assert!(
            window[0].sequence < window[1].sequence,
            "events should be in sequence order"
        );
    }

    Ok(())
}
