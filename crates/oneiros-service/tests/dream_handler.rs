mod common;
use common::*;

#[tokio::test]
async fn dream_returns_agent_context() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "governor", "process").await;
    seed_texture(&state, &token, "working").await;

    let _cog = create_cognition(&state, &token, "governor", "working", "A test thought.").await;

    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/governor", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(context.agent.name, AgentName::new("governor"));
    assert_eq!(context.cognitions.len(), 1);
}

/// Connections are scoped to an agent's entities (memories + experiences).
/// Each agent sees only the connections touching their own identity graph.
#[tokio::test]
async fn dream_scopes_connections_to_agent() {
    let (_temp, state, token) = setup();

    // Set up two agents with a shared persona.
    seed_agent(&state, &token, "agent-a", "process").await;
    seed_agent(&state, &token, "agent-b", "process").await;
    seed_texture(&state, &token, "working").await;
    seed_nature(&state, &token, "reference").await;

    // Create memories for each agent (memories anchor the graph).
    let mem_a = create_memory(&state, &token, "agent-a", "session", "Memory from A.").await;
    let mem_b = create_memory(&state, &token, "agent-b", "session", "Memory from B.").await;

    // Create cognitions for each agent.
    let cog_a = create_cognition(&state, &token, "agent-a", "working", "Thought from A.").await;
    let cog_b = create_cognition(&state, &token, "agent-b", "working", "Thought from B.").await;

    // Connect each agent's memory to their cognition.
    let _conn_a = create_connection(
        &state,
        &token,
        "reference",
        &Ref::memory(mem_a.id),
        &Ref::cognition(cog_a.id),
    )
    .await;

    let _conn_b = create_connection(
        &state,
        &token,
        "reference",
        &Ref::memory(mem_b.id),
        &Ref::cognition(cog_b.id),
    )
    .await;

    // Dream agent-a: should see only agent-a's connection.
    let app = router(state.clone());
    let response = app
        .oneshot(post_auth("/dream/agent-a", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(
        context.connections.len(),
        1,
        "agent-a should see 1 connection, not both"
    );
    assert_eq!(context.connections[0].from_ref, Ref::memory(mem_a.id));

    // Dream agent-b: should see only agent-b's connection.
    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/agent-b", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(
        context.connections.len(),
        1,
        "agent-b should see 1 connection, not both"
    );
    assert_eq!(context.connections[0].from_ref, Ref::memory(mem_b.id));
}

/// When connections exist, the dream should include cognitions that are
/// reachable via those connections — not ALL cognitions.
#[tokio::test]
async fn dream_collector_returns_connected_cognitions() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "dreamer", "process").await;
    seed_texture(&state, &token, "working").await;
    seed_nature(&state, &token, "reference").await;

    // Create a memory so connections are scoped to this agent.
    let mem = create_memory(&state, &token, "dreamer", "session", "Test memory.").await;

    // Create cognitions — one connected via the memory, one not.
    let connected_cog =
        create_cognition(&state, &token, "dreamer", "working", "Connected thought.").await;
    let _unconnected_cog =
        create_cognition(&state, &token, "dreamer", "working", "Unconnected thought.").await;

    // Connect the memory to one cognition.
    create_connection(
        &state,
        &token,
        "reference",
        &Ref::memory(mem.id),
        &Ref::cognition(connected_cog.id),
    )
    .await;

    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/dreamer", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    // Graph traversal is active (connections exist), so not all cognitions
    // are included — only connected + recent 20.
    // Both cognitions are recent (< 20), so both appear via the recent window.
    // The important thing: the collector used graph traversal, not full dump.
    assert!(!context.cognitions.is_empty());
    assert!(context.cognitions.iter().any(|c| c.id == connected_cog.id));

    // Connections should be present — scoped to this agent's entities.
    assert_eq!(context.connections.len(), 1);
}

/// With no connections at all, the dream falls back to ALL cognitions.
#[tokio::test]
async fn dream_collector_falls_back_for_empty_graph() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "sparse", "process").await;
    seed_texture(&state, &token, "working").await;

    // Create several cognitions but NO connections.
    for i in 0..5 {
        create_cognition(&state, &token, "sparse", "working", &format!("Thought {i}")).await;
    }

    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/sparse", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    // Fallback: ALL cognitions included.
    assert_eq!(context.cognitions.len(), 5);
    assert!(context.connections.is_empty());
}

/// Memories at or above the recollection_level threshold are included.
/// Core memories are always included regardless of threshold.
/// Default threshold is "session", so session-level memories pass.
#[tokio::test]
async fn dream_collector_includes_memories_at_threshold() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "rememberer", "process").await;
    seed_texture(&state, &token, "working").await;

    // Create memories at session level (default threshold).
    let _mem1 = create_memory(&state, &token, "rememberer", "session", "First memory.").await;
    let _mem2 = create_memory(&state, &token, "rememberer", "session", "Second memory.").await;
    let _mem3 = create_memory(&state, &token, "rememberer", "session", "Third memory.").await;

    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/rememberer", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    // Session-level memories pass the default "session" threshold.
    assert_eq!(context.memories.len(), 3);
}

/// Recent cognitions appear in the dream even when not connected via the
/// graph, ensuring the agent has current-session context for continuity.
#[tokio::test]
async fn dream_collector_includes_recent_cognitions() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "oriented", "process").await;
    seed_texture(&state, &token, "working").await;
    seed_nature(&state, &token, "reference").await;

    // Create a memory so the collector can find connections.
    let mem = create_memory(&state, &token, "oriented", "session", "Anchor.").await;

    // Create a connected cognition (reachable via the graph).
    let connected =
        create_cognition(&state, &token, "oriented", "working", "Graph-connected.").await;
    create_connection(
        &state,
        &token,
        "reference",
        &Ref::memory(mem.id),
        &Ref::cognition(connected.id),
    )
    .await;

    // Create a recent cognition that is NOT connected.
    let recent =
        create_cognition(&state, &token, "oriented", "working", "Recent unconnected.").await;

    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/oriented", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    // Both the connected cognition and the recent one should be present.
    let ids: Vec<CognitionId> = context.cognitions.iter().map(|c| c.id).collect();
    assert!(
        ids.contains(&connected.id),
        "connected cognition should be in dream"
    );
    assert!(
        ids.contains(&recent.id),
        "recent cognition should be in dream"
    );
}

/// dream_depth limits how far the BFS traverses from the seed set.
#[tokio::test]
async fn dream_max_depth_limits_traversal() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "deep", "process").await;
    seed_texture(&state, &token, "working").await;
    seed_nature(&state, &token, "reference").await;

    let mem = create_memory(&state, &token, "deep", "session", "Root.").await;

    // Chain: memory → cog_a (depth 1) → cog_b (depth 2).
    let cog_a = create_cognition(&state, &token, "deep", "working", "Depth one.").await;
    let cog_b = create_cognition(&state, &token, "deep", "working", "Depth two.").await;

    create_connection(
        &state,
        &token,
        "reference",
        &Ref::memory(mem.id),
        &Ref::cognition(cog_a.id),
    )
    .await;
    create_connection(
        &state,
        &token,
        "reference",
        &Ref::cognition(cog_a.id),
        &Ref::cognition(cog_b.id),
    )
    .await;

    // dream_depth=1, recent_window=0: only cog_a reachable.
    let app = router(state.clone());
    let response = app
        .oneshot(post_auth(
            "/dream/deep?dream_depth=1&recent_window=0",
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    let ids: Vec<CognitionId> = context.cognitions.iter().map(|c| c.id).collect();
    assert!(
        ids.contains(&cog_a.id),
        "depth-1 cognition should be in dream"
    );
    assert!(
        !ids.contains(&cog_b.id),
        "depth-2 cognition should NOT be in dream at dream_depth=1"
    );

    // Explicit deeper limit: both reachable.
    let app = router(state);
    let response = app
        .oneshot(post_auth(
            "/dream/deep?dream_depth=10&recent_window=0",
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    let ids: Vec<CognitionId> = context.cognitions.iter().map(|c| c.id).collect();
    assert!(
        ids.contains(&cog_a.id),
        "depth-1 cognition should be in dream"
    );
    assert!(
        ids.contains(&cog_b.id),
        "depth-2 cognition should be in dream with deeper limit"
    );
}

/// cognition_size caps the total number of cognitions in the dream.
#[tokio::test]
async fn dream_max_cognitions_caps_output() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "capped", "process").await;
    seed_texture(&state, &token, "working").await;
    seed_nature(&state, &token, "reference").await;

    let mem = create_memory(&state, &token, "capped", "session", "Anchor.").await;

    // Create 5 cognitions, all directly connected to the memory.
    for i in 0..5 {
        let cog =
            create_cognition(&state, &token, "capped", "working", &format!("Thought {i}")).await;
        create_connection(
            &state,
            &token,
            "reference",
            &Ref::memory(mem.id),
            &Ref::cognition(cog.id),
        )
        .await;
    }

    // Cap at 2 cognitions, recent_window=0 so only graph-discovered appear.
    let app = router(state.clone());
    let response = app
        .oneshot(post_auth(
            "/dream/capped?cognition_size=2&recent_window=0",
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(context.cognitions.len(), 2, "should cap at cognition_size");
    // Connections should be filtered to match — only 2 connections remain.
    assert_eq!(
        context.connections.len(),
        2,
        "connections should be filtered to match included cognitions"
    );
}

/// recollection_level filters memories by level priority.
/// Priority order: core > working > session > project > archival.
/// Core memories are always included. Setting level=project includes
/// core + working + session + project, excludes archival.
#[tokio::test]
async fn dream_filters_memories_by_level() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "leveled", "process").await;
    seed_texture(&state, &token, "working").await;

    // Create memories at different levels.
    let _core = create_memory(&state, &token, "leveled", "core", "Core identity.").await;
    let _project = create_memory(&state, &token, "leveled", "project", "Project insight.").await;
    let _working = create_memory(&state, &token, "leveled", "working", "Working note.").await;
    let _archival =
        create_memory(&state, &token, "leveled", "archival", "Historical context.").await;

    // Dream with recollection_level=project: core + working + project included, archival excluded.
    let app = router(state);
    let response = app
        .oneshot(post_auth(
            "/dream/leveled?recollection_level=project",
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(
        context.memories.len(),
        3,
        "core + working + project included, archival excluded"
    );

    let levels: Vec<&str> = context.memories.iter().map(|m| m.level.as_ref()).collect();
    assert!(levels.contains(&"core"));
    assert!(levels.contains(&"working"));
    assert!(levels.contains(&"project"));
    assert!(!levels.contains(&"archival"));
}

/// recollection_size caps non-core memories. Core memories always survive.
#[tokio::test]
async fn dream_caps_non_core_memories() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "capped-mem", "process").await;
    seed_texture(&state, &token, "working").await;

    // Create 2 core memories and 5 project memories.
    let _core1 = create_memory(&state, &token, "capped-mem", "core", "Core one.").await;
    let _core2 = create_memory(&state, &token, "capped-mem", "core", "Core two.").await;
    for i in 0..5 {
        create_memory(
            &state,
            &token,
            "capped-mem",
            "project",
            &format!("Project {i}."),
        )
        .await;
    }

    // Cap non-core at 3, level threshold includes project.
    let app = router(state);
    let response = app
        .oneshot(post_auth(
            "/dream/capped-mem?recollection_size=3&recollection_level=project",
            &token,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    // 2 core + 3 capped project = 5 total.
    assert_eq!(context.memories.len(), 5, "2 core + 3 capped project");

    let core_count = context
        .memories
        .iter()
        .filter(|m| m.level.as_ref() == "core")
        .count();
    let project_count = context
        .memories
        .iter()
        .filter(|m| m.level.as_ref() == "project")
        .count();
    assert_eq!(core_count, 2, "all core memories survive");
    assert_eq!(project_count, 3, "project capped at recollection_size");
}

/// experience_size caps the total number of experiences in the dream.
#[tokio::test]
async fn dream_caps_experiences() {
    let (_temp, state, token) = setup();
    seed_agent(&state, &token, "exp-capped", "process").await;
    seed_texture(&state, &token, "working").await;
    seed_sensation(&state, &token, "continues").await;

    // Create 5 experiences.
    for i in 0..5 {
        create_experience(
            &state,
            &token,
            "exp-capped",
            "continues",
            &format!("Thread {i}."),
        )
        .await;
    }

    // Cap at 2.
    let app = router(state);
    let response = app
        .oneshot(post_auth("/dream/exp-capped?experience_size=2", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let context: DreamContext = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(
        context.experiences.len(),
        2,
        "should cap at experience_size"
    );
}
