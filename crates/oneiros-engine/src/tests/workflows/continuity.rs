//! Continuity workflow — the core cognitive loop.
//!
//! An agent wakes, thinks, remembers, connects ideas, dreams, and sleeps.
//! This is the heart of oneiros: the cycle from raw thought to
//! consolidated understanding, all assembled through dreaming.

use futures::StreamExt;

use crate::tests::harness::{Retryable, TestApp};
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
            filters: SearchFilters::default(),
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
            assert_eq!(m.data.level.as_str(), "core");
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
            filters: SearchFilters::default(),
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
            filters: SearchFilters::default(),
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
                .from_ref(RefToken::new(Ref::memory(core_memory.data.id)))
                .to_ref(RefToken::new(Ref::experience(experience.data.id)))
                .nature("context")
                .build(),
        )
        .await?;

    match client
        .connection()
        .list(&ListConnections {
            entity: None,
            filters: SearchFilters::default(),
        })
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
    match client.agent().get(&agent).await? {
        AgentResponse::AgentDetails(a) => {
            assert_eq!(a.data.persona, PersonaName::new("custom"));
        }
        other => panic!("expected AgentDetails, got {other:?}"),
    }

    // Status — via continuity client
    match client.continuity().status().await? {
        ContinuityResponse::Status(table) => {
            assert!(!table.agents.is_empty(), "should have agents in status");
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

    // Subscribe to the SSE activity stream in a background task.
    // Use a oneshot to ensure the connection is established before
    // firing commands — otherwise events can be broadcast before the
    // server-side receiver exists, causing a heisenfail.
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<StoredEvent>(64);
    let (connected_tx, connected_rx) = tokio::sync::oneshot::channel::<Result<(), String>>();

    let sse_handle = tokio::spawn(async move {
        let http = reqwest::Client::new();
        let resp = http
            .get(&sse_url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Accept", "text/event-stream")
            .send()
            .await
            .unwrap();

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            let _ = connected_tx.send(Err(format!("header auth failed: {status}\n{body}")));
            return;
        }

        let _ = connected_tx.send(Ok(()));
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

    // Wait for the SSE connection to be established before emitting events
    match connected_rx.await {
        Ok(Ok(())) => {}
        Ok(Err(msg)) => panic!("{}", msg),
        Err(_) => panic!("SSE task should signal connection"),
    }

    // Perform operations that should emit events
    app.command("agent create thinker process").await?;
    app.command(r#"cognition add thinker.process observation "A thought""#)
        .await?;
    app.command(r#"memory add thinker.process session "A memory""#)
        .await?;

    let expected = ["agent-created", "cognition-added", "memory-added"];
    let mut events = Vec::new();

    Retryable::default()
        .wait_for(
            || {
                while let Ok(event) = event_rx.try_recv() {
                    events.push(event);
                }
                let types: Vec<String> = events.iter().map(|e| e.data.event_type()).collect();
                if expected.iter().all(|e| types.iter().any(|t| t == e)) {
                    Ok(())
                } else {
                    Err(format!("got: {types:?}"))
                }
            },
            "all expected SSE event types to arrive",
        )
        .await;

    sse_handle.abort();

    // Events should be in sequence order
    for window in events.windows(2) {
        assert!(
            window[0].sequence < window[1].sequence,
            "events should be in sequence order"
        );
    }

    Ok(())
}

/// Auth via query param — the path browsers must use for SSE since
/// EventSource cannot set custom headers.
#[tokio::test]
async fn activity_stream_authenticates_via_query_param() -> Result<(), Box<dyn core::error::Error>>
{
    let app = TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await?;

    let token = app.token().expect("should have a token after init");
    let sse_url = format!("{}/activity?token={token}", app.base_url());

    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel::<StoredEvent>(64);
    let (connected_tx, connected_rx) = tokio::sync::oneshot::channel::<Result<(), String>>();

    let sse_handle = tokio::spawn(async move {
        let http = reqwest::Client::new();
        // No Authorization header — token is in the query string
        let resp = http
            .get(&sse_url)
            .header("Accept", "text/event-stream")
            .send()
            .await
            .unwrap();

        // Check status and signal result to main thread
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            let error_msg = format!("query param auth failed: {status}\n{body}");
            let _ = connected_tx.send(Err(error_msg));
            return;
        }

        let _ = connected_tx.send(Ok(()));
        let mut stream = resp.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk) = stream.next().await {
            let bytes = chunk.unwrap();
            let text = String::from_utf8_lossy(&bytes);
            buffer.push_str(&text);

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

    // Wait for the SSE connection to be established before emitting events
    match connected_rx.await {
        Ok(Ok(())) => {}                                       // Success
        Ok(Err(msg)) => panic!("{}", msg),                     // Auth failed with details
        Err(_) => panic!("SSE task should signal connection"), // Task crashed
    }

    app.command("agent create thinker process").await?;

    let mut events = Vec::<StoredEvent>::new();

    Retryable::default()
        .wait_for(
            || {
                while let Ok(event) = event_rx.try_recv() {
                    events.push(event);
                }
                if events.is_empty() {
                    Err("no events received yet".into())
                } else {
                    Ok(())
                }
            },
            "SSE events to arrive via query param auth",
        )
        .await;

    sse_handle.abort();

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
            &EmergeAgent::builder()
                .name("thinker")
                .persona("process")
                .build(),
        )
        .await?;

    // Add a cognition and a memory
    app.command(r#"cognition add thinker.process observation "A thought worth connecting""#)
        .await?;
    app.command(r#"memory add thinker.process session "A memory worth connecting""#)
        .await?;

    // List cognitions — extract ref from response meta, not from raw ID
    let cognition_ref = match client
        .cognition()
        .list(&ListCognitions {
            agent: Some(agent.clone()),
            texture: None,
            filters: SearchFilters::default(),
        })
        .await?
    {
        CognitionResponse::Cognitions(listed) => {
            let first = &listed.items[0];
            first
                .meta
                .as_ref()
                .and_then(|m| m.ref_token.as_ref())
                .expect("listed cognition should carry ref_token in meta")
                .clone()
        }
        other => panic!("expected Cognitions, got {other:?}"),
    };

    // List memories — same pattern
    let memory_ref = match client
        .memory()
        .list(&ListMemories {
            agent: Some(agent.clone()),
            filters: SearchFilters::default(),
        })
        .await?
    {
        MemoryResponse::Memories(listed) => {
            let first = &listed.items[0];
            first
                .meta
                .as_ref()
                .and_then(|m| m.ref_token.as_ref())
                .expect("listed memory should carry ref_token in meta")
                .clone()
        }
        other => panic!("expected Memories, got {other:?}"),
    };

    // Connect using only the refs we extracted — no manual RefToken construction
    client
        .connection()
        .create(
            &CreateConnection::builder()
                .from_ref(cognition_ref)
                .to_ref(memory_ref)
                .nature("context")
                .build(),
        )
        .await?;

    // Verify the connection exists
    match client
        .connection()
        .list(&ListConnections {
            entity: None,
            filters: SearchFilters::default(),
        })
        .await?
    {
        ConnectionResponse::Connections(conns) => assert_eq!(conns.len(), 1),
        other => panic!("expected Connections, got {other:?}"),
    }

    Ok(())
}
