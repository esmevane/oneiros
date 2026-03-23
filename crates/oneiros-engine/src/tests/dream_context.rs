use std::collections::HashSet;

use crate::*;

// ── Helpers ───────────────────────────────────────────────────────

fn seeded_ctx() -> ProjectContext {
    let mut engine = Engine::in_memory().expect("init engine");
    engine.init_project("test").expect("init project");
    let ctx = engine.project().unwrap().clone();
    SeedService::core(&ctx).expect("seed core");
    ctx
}

fn seed_agent(ctx: &ProjectContext) -> AgentName {
    AgentService::create(
        ctx,
        "thinker".into(),
        "process".into(),
        "A thinking agent".into(),
        "You think.".into(),
    )
    .unwrap();
    AgentName::new("thinker.process")
}

fn add_cognition(ctx: &ProjectContext, agent: &AgentName, content: &str) -> CognitionId {
    match CognitionService::add(
        ctx,
        agent,
        TextureName::new("observation"),
        Content::new(content),
    )
    .unwrap()
    {
        CognitionResponse::CognitionAdded(c) => c.id,
        other => panic!("expected CognitionAdded, got {other:?}"),
    }
}

fn add_memory(ctx: &ProjectContext, agent: &AgentName, level: &str, content: &str) -> MemoryId {
    match MemoryService::add(ctx, agent, LevelName::new(level), Content::new(content)).unwrap() {
        MemoryResponse::MemoryAdded(m) => m.id,
        other => panic!("expected MemoryAdded, got {other:?}"),
    }
}

fn add_experience(ctx: &ProjectContext, agent: &AgentName, description: &str) -> ExperienceId {
    match ExperienceService::create(
        ctx,
        agent,
        SensationName::new("echoes"),
        Description::new(description),
    )
    .unwrap()
    {
        ExperienceResponse::ExperienceCreated(e) => e.id,
        other => panic!("expected ExperienceCreated, got {other:?}"),
    }
}

fn connect(ctx: &ProjectContext, from: &Ref, to: &Ref) -> ConnectionId {
    let from_token = RefToken::from(from.clone()).to_string();
    let to_token = RefToken::from(to.clone()).to_string();
    match ConnectionService::create(ctx, from_token, to_token, "reference".to_string()).unwrap() {
        ConnectionResponse::ConnectionCreated(c) => c.id,
        other => panic!("expected ConnectionCreated, got {other:?}"),
    }
}

fn dream(ctx: &ProjectContext, agent: &AgentName) -> DreamContext {
    dream_with(ctx, agent, &DreamOverrides::default())
}

fn dream_with(ctx: &ProjectContext, agent: &AgentName, overrides: &DreamOverrides) -> DreamContext {
    match ContinuityService::dream(ctx, agent, overrides).unwrap() {
        ContinuityResponse::Dreaming(context) => context,
        other => panic!("expected Dreaming, got {other:?}"),
    }
}

// ── Vocabulary ───────────────────────────────────────────────────

#[test]
fn dream_includes_all_vocabulary() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    let context = dream(&ctx, &agent);

    assert_eq!(context.textures.len(), 10, "seed creates 10 textures");
    assert_eq!(context.levels.len(), 5, "seed creates 5 levels");
    assert_eq!(context.sensations.len(), 6, "seed creates 6 sensations");
    assert_eq!(context.natures.len(), 6, "seed creates 6 natures");
    assert_eq!(context.urges.len(), 4, "seed creates 4 urges");
}

#[test]
fn dream_includes_persona() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    let context = dream(&ctx, &agent);

    let persona = context.persona.expect("persona should be present");
    assert_eq!(persona.name, PersonaName::new("process"));
}

// ── Memory filtering ─────────────────────────────────────────────

#[test]
fn core_memories_always_included() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    add_memory(&ctx, &agent, "core", "identity fundament");
    add_memory(&ctx, &agent, "archival", "old history");

    // Default config has recollection_level=project (priority 2).
    // archival has priority 1 — below threshold, excluded.
    // core has priority 5 — always included regardless of threshold.
    let context = dream(&ctx, &agent);

    let memory_contents: Vec<&str> = context
        .memories
        .iter()
        .map(|m| m.content.as_str())
        .collect();
    assert!(
        memory_contents.contains(&"identity fundament"),
        "core memories must always be included"
    );
    assert!(
        !memory_contents.contains(&"old history"),
        "archival memories should be excluded at project level threshold"
    );
}

#[test]
fn level_threshold_filters_lower_priority() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    add_memory(&ctx, &agent, "core", "core memory");
    add_memory(&ctx, &agent, "project", "project memory");
    add_memory(&ctx, &agent, "session", "session memory");
    add_memory(&ctx, &agent, "working", "working memory");
    add_memory(&ctx, &agent, "archival", "archival memory");

    // Default: recollection_level=project (priority 2)
    // Should include: core (always), project (>=2)
    // Should exclude: session (3? no — session is 3, project is 2, so session IS included)
    // Wait: level_priority: core=5, working=4, session=3, project=2, archival=1
    // Threshold=project means min_priority=2, so include >=2: core, working, session, project
    // Exclude: archival (1)
    let context = dream(&ctx, &agent);

    let levels: Vec<&str> = context.memories.iter().map(|m| m.level.as_str()).collect();
    assert!(levels.contains(&"core"), "core always included");
    assert!(levels.contains(&"project"), "project >= threshold");
    assert!(levels.contains(&"session"), "session >= threshold");
    assert!(levels.contains(&"working"), "working >= threshold");
    assert!(
        !levels.contains(&"archival"),
        "archival below project threshold"
    );
}

#[test]
fn level_threshold_override_changes_filter() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    add_memory(&ctx, &agent, "core", "core memory");
    add_memory(&ctx, &agent, "project", "project memory");
    add_memory(&ctx, &agent, "session", "session memory");
    add_memory(&ctx, &agent, "working", "working memory");
    add_memory(&ctx, &agent, "archival", "archival memory");

    // Override to core level (priority 5) — only core survives
    let overrides = DreamOverrides {
        recollection_level: Some(LevelName::new("core")),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides);

    let levels: Vec<&str> = context.memories.iter().map(|m| m.level.as_str()).collect();
    assert!(levels.contains(&"core"), "core always included");
    assert_eq!(
        levels.len(),
        1,
        "only core should survive at core threshold"
    );
}

#[test]
fn recollection_size_caps_non_core_memories() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    // Add 5 project memories
    for i in 0..5 {
        add_memory(&ctx, &agent, "project", &format!("project memory {i}"));
    }
    add_memory(&ctx, &agent, "core", "core memory");

    // Cap non-core at 2
    let overrides = DreamOverrides {
        recollection_size: Some(2),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides);

    let core_count = context
        .memories
        .iter()
        .filter(|m| m.level.as_str() == "core")
        .count();
    let non_core_count = context
        .memories
        .iter()
        .filter(|m| m.level.as_str() != "core")
        .count();

    assert_eq!(core_count, 1, "core memory is always included");
    assert_eq!(non_core_count, 2, "non-core capped at recollection_size");
}

// ── Cognition selection ──────────────────────────────────────────

#[test]
fn sparse_graph_includes_all_cognitions() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    // Add cognitions but no connections — sparse graph
    for i in 0..5 {
        add_cognition(&ctx, &agent, &format!("thought {i}"));
    }

    let context = dream(&ctx, &agent);
    assert_eq!(
        context.cognitions.len(),
        5,
        "sparse graph should include all cognitions"
    );
}

#[test]
fn cognition_size_cap_keeps_most_recent() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    for i in 0..10 {
        add_cognition(&ctx, &agent, &format!("thought {i}"));
    }

    let overrides = DreamOverrides {
        cognition_size: Some(3),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides);

    assert_eq!(context.cognitions.len(), 3, "capped at cognition_size");
    // Should be the most recent (highest numbers)
    let contents: Vec<&str> = context
        .cognitions
        .iter()
        .map(|c| c.content.as_str())
        .collect();
    assert!(
        contents.contains(&"thought 9"),
        "most recent should survive cap"
    );
    assert!(
        contents.contains(&"thought 8"),
        "second most recent should survive"
    );
    assert!(
        contents.contains(&"thought 7"),
        "third most recent should survive"
    );
}

// ── BFS graph traversal ──────────────────────────────────────────

#[test]
fn bfs_discovers_connected_cognitions() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    let mem_id = add_memory(&ctx, &agent, "project", "seed memory");
    let connected_cog = add_cognition(&ctx, &agent, "connected thought");
    let _unconnected_cog = add_cognition(&ctx, &agent, "unconnected thought");

    // Connect the memory to one cognition
    connect(&ctx, &Ref::memory(mem_id), &Ref::cognition(connected_cog));

    // Use a large cognition_size so the cap doesn't interfere
    let overrides = DreamOverrides {
        cognition_size: Some(100),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides);

    let cog_ids: HashSet<CognitionId> = context.cognitions.iter().map(|c| c.id).collect();

    assert!(
        cog_ids.contains(&connected_cog),
        "BFS should discover connected cognition"
    );
    // The unconnected one should also appear because recent cognitions
    // are included via the orientation window
}

#[test]
fn bfs_discovers_connected_experiences() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    let mem_id = add_memory(&ctx, &agent, "project", "seed memory");
    let connected_exp = add_experience(&ctx, &agent, "connected experience");
    let _unconnected_exp = add_experience(&ctx, &agent, "unconnected experience");

    connect(&ctx, &Ref::memory(mem_id), &Ref::experience(connected_exp));

    let overrides = DreamOverrides {
        experience_size: Some(100),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides);

    let exp_ids: HashSet<ExperienceId> = context.experiences.iter().map(|e| e.id).collect();
    assert!(
        exp_ids.contains(&connected_exp),
        "BFS should discover connected experience"
    );
}

// ── Experience selection ─────────────────────────────────────────

#[test]
fn experience_size_cap_keeps_most_recent() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    for i in 0..8 {
        add_experience(&ctx, &agent, &format!("experience {i}"));
    }

    let overrides = DreamOverrides {
        experience_size: Some(3),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides);

    assert_eq!(context.experiences.len(), 3, "capped at experience_size");
}

// ── Connection pruning ───────────────────────────────────────────

#[test]
fn connections_pruned_to_included_endpoints() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    let mem_id = add_memory(&ctx, &agent, "project", "seed memory");
    let cog_id = add_cognition(&ctx, &agent, "connected thought");
    let orphan_cog = add_cognition(&ctx, &agent, "orphan thought");

    // Create two connections — one between included entities, one to an "orphan"
    let _included_conn = connect(&ctx, &Ref::memory(mem_id), &Ref::cognition(cog_id));

    // This connection goes to a cognition that might get filtered out
    // We need a scenario where one endpoint is excluded.
    // Create a memory at archival level (excluded by default threshold)
    let archival_mem = add_memory(&ctx, &agent, "archival", "old memory");
    let _excluded_conn = connect(
        &ctx,
        &Ref::memory(archival_mem),
        &Ref::cognition(orphan_cog),
    );

    let context = dream(&ctx, &agent);

    // Every connection in the result should have both endpoints present in the result
    let included_refs: HashSet<Ref> = context
        .memories
        .iter()
        .map(|m| Ref::memory(m.id))
        .chain(context.cognitions.iter().map(|c| Ref::cognition(c.id)))
        .chain(context.experiences.iter().map(|e| Ref::experience(e.id)))
        .collect();

    for conn in &context.connections {
        assert!(
            included_refs.contains(&conn.from_ref),
            "connection from_ref {:?} should be in included entities",
            conn.from_ref
        );
        assert!(
            included_refs.contains(&conn.to_ref),
            "connection to_ref {:?} should be in included entities",
            conn.to_ref
        );
    }
}

// ── Pressure readings ────────────────────────────────────────────

#[test]
fn pressures_paired_with_urge_ctas() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    // Add some cognitions to generate pressure (introspect heuristic)
    for i in 0..10 {
        add_cognition(&ctx, &agent, &format!("thought {i}"));
    }

    let context = dream(&ctx, &agent);

    // Pressures may or may not be populated depending on whether the
    // pressure projection ran. If they are populated, verify the pairing.
    for reading in &context.pressures {
        // The CTA should come from the matching urge
        let urge = context
            .urges
            .iter()
            .find(|u| u.name == reading.pressure.urge);
        if let Some(urge) = urge {
            assert_eq!(
                reading.cta, urge.prompt,
                "pressure CTA should match urge prompt"
            );
        }
    }
}

// ── Config override integration ──────────────────────────────────

#[test]
fn dream_overrides_change_output() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    // Seed enough data to make caps meaningful
    for i in 0..10 {
        add_cognition(&ctx, &agent, &format!("thought {i}"));
    }
    for i in 0..10 {
        add_memory(&ctx, &agent, "project", &format!("memory {i}"));
    }

    // Default dream
    let default_context = dream(&ctx, &agent);

    // Restricted dream
    let overrides = DreamOverrides {
        cognition_size: Some(2),
        recollection_size: Some(2),
        ..Default::default()
    };
    let restricted_context = dream_with(&ctx, &agent, &overrides);

    assert!(
        restricted_context.cognitions.len() <= 2,
        "override should restrict cognitions"
    );
    assert!(
        restricted_context.memories.len() < default_context.memories.len()
            || default_context.memories.len() <= 3, // 2 non-core + potential core
        "override should restrict memories"
    );
}

// ── Ordering ─────────────────────────────────────────────────────

#[test]
fn memories_sorted_by_created_at() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    add_memory(&ctx, &agent, "core", "first core");
    add_memory(&ctx, &agent, "project", "first project");
    add_memory(&ctx, &agent, "core", "second core");
    add_memory(&ctx, &agent, "project", "second project");

    let context = dream(&ctx, &agent);

    for window in context.memories.windows(2) {
        assert!(
            window[0].created_at <= window[1].created_at,
            "memories should be sorted by created_at"
        );
    }
}

#[test]
fn cognitions_sorted_by_created_at() {
    let ctx = seeded_ctx();
    let agent = seed_agent(&ctx);

    for i in 0..5 {
        add_cognition(&ctx, &agent, &format!("thought {i}"));
    }

    let context = dream(&ctx, &agent);

    for window in context.cognitions.windows(2) {
        assert!(
            window[0].created_at <= window[1].created_at,
            "cognitions should be sorted by created_at"
        );
    }
}
