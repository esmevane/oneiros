use std::collections::HashSet;

use crate::*;

// ── Helpers ───────────────────────────────────────────────────────

async fn seeded_ctx() -> ProjectContext {
    let mut engine = Engine::in_memory().expect("init engine");
    engine.init_project("test").expect("init project");
    let ctx = engine.project().unwrap().clone();
    SeedService::core(&ctx).await.expect("seed core");
    ctx
}

async fn seed_agent(ctx: &ProjectContext) -> AgentName {
    AgentService::create(
        ctx,
        "thinker".into(),
        "process".into(),
        "A thinking agent".into(),
        "You think.".into(),
    )
    .await
    .unwrap();
    AgentName::new("thinker.process")
}

async fn add_cognition(ctx: &ProjectContext, agent: &AgentName, content: &str) -> CognitionId {
    match CognitionService::add(
        ctx,
        agent,
        TextureName::new("observation"),
        Content::new(content),
    )
    .await
    .unwrap()
    {
        CognitionResponse::CognitionAdded(c) => c.id,
        other => panic!("expected CognitionAdded, got {other:?}"),
    }
}

async fn add_memory(
    ctx: &ProjectContext,
    agent: &AgentName,
    level: &str,
    content: &str,
) -> MemoryId {
    match MemoryService::add(ctx, agent, LevelName::new(level), Content::new(content))
        .await
        .unwrap()
    {
        MemoryResponse::MemoryAdded(m) => m.id,
        other => panic!("expected MemoryAdded, got {other:?}"),
    }
}

async fn add_experience(
    ctx: &ProjectContext,
    agent: &AgentName,
    description: &str,
) -> ExperienceId {
    match ExperienceService::create(
        ctx,
        agent,
        SensationName::new("echoes"),
        Description::new(description),
    )
    .await
    .unwrap()
    {
        ExperienceResponse::ExperienceCreated(e) => e.id,
        other => panic!("expected ExperienceCreated, got {other:?}"),
    }
}

async fn connect(ctx: &ProjectContext, from: &Ref, to: &Ref) -> ConnectionId {
    let from_token = RefToken::from(from.clone()).to_string();
    let to_token = RefToken::from(to.clone()).to_string();
    match ConnectionService::create(ctx, from_token, to_token, "reference".to_string())
        .await
        .unwrap()
    {
        ConnectionResponse::ConnectionCreated(c) => c.id,
        other => panic!("expected ConnectionCreated, got {other:?}"),
    }
}

async fn dream(ctx: &ProjectContext, agent: &AgentName) -> DreamContext {
    dream_with(ctx, agent, &DreamOverrides::default()).await
}

async fn dream_with(
    ctx: &ProjectContext,
    agent: &AgentName,
    overrides: &DreamOverrides,
) -> DreamContext {
    match ContinuityService::dream(ctx, agent, overrides)
        .await
        .unwrap()
    {
        ContinuityResponse::Dreaming(context) => context,
        other => panic!("expected Dreaming, got {other:?}"),
    }
}

// ── Vocabulary ───────────────────────────────────────────────────

#[tokio::test]
async fn dream_includes_all_vocabulary() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    let context = dream(&ctx, &agent).await;

    assert_eq!(context.textures.len(), 10, "seed creates 10 textures");
    assert_eq!(context.levels.len(), 5, "seed creates 5 levels");
    assert_eq!(context.sensations.len(), 6, "seed creates 6 sensations");
    assert_eq!(context.natures.len(), 6, "seed creates 6 natures");
    assert_eq!(context.urges.len(), 4, "seed creates 4 urges");
}

#[tokio::test]
async fn dream_includes_persona() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    let context = dream(&ctx, &agent).await;

    let persona = context.persona.expect("persona should be present");
    assert_eq!(persona.name, PersonaName::new("process"));
}

// ── Memory filtering ─────────────────────────────────────────────

#[tokio::test]
async fn core_memories_always_included() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    add_memory(&ctx, &agent, "core", "identity fundament").await;
    add_memory(&ctx, &agent, "archival", "old history").await;

    let context = dream(&ctx, &agent).await;

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

#[tokio::test]
async fn level_threshold_filters_lower_priority() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    add_memory(&ctx, &agent, "core", "core memory").await;
    add_memory(&ctx, &agent, "project", "project memory").await;
    add_memory(&ctx, &agent, "session", "session memory").await;
    add_memory(&ctx, &agent, "working", "working memory").await;
    add_memory(&ctx, &agent, "archival", "archival memory").await;

    let context = dream(&ctx, &agent).await;

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

#[tokio::test]
async fn level_threshold_override_changes_filter() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    add_memory(&ctx, &agent, "core", "core memory").await;
    add_memory(&ctx, &agent, "project", "project memory").await;
    add_memory(&ctx, &agent, "session", "session memory").await;
    add_memory(&ctx, &agent, "working", "working memory").await;
    add_memory(&ctx, &agent, "archival", "archival memory").await;

    let overrides = DreamOverrides {
        recollection_level: Some(LevelName::new("core")),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides).await;

    let levels: Vec<&str> = context.memories.iter().map(|m| m.level.as_str()).collect();
    assert!(levels.contains(&"core"), "core always included");
    assert_eq!(
        levels.len(),
        1,
        "only core should survive at core threshold"
    );
}

#[tokio::test]
async fn recollection_size_caps_non_core_memories() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    for i in 0..5 {
        add_memory(&ctx, &agent, "project", &format!("project memory {i}")).await;
    }
    add_memory(&ctx, &agent, "core", "core memory").await;

    let overrides = DreamOverrides {
        recollection_size: Some(2),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides).await;

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

#[tokio::test]
async fn sparse_graph_includes_all_cognitions() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    for i in 0..5 {
        add_cognition(&ctx, &agent, &format!("thought {i}")).await;
    }

    let context = dream(&ctx, &agent).await;
    assert_eq!(
        context.cognitions.len(),
        5,
        "sparse graph should include all cognitions"
    );
}

#[tokio::test]
async fn cognition_size_cap_keeps_most_recent() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    for i in 0..10 {
        add_cognition(&ctx, &agent, &format!("thought {i}")).await;
    }

    let overrides = DreamOverrides {
        cognition_size: Some(3),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides).await;

    assert_eq!(context.cognitions.len(), 3, "capped at cognition_size");
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

#[tokio::test]
async fn bfs_discovers_connected_cognitions() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    let mem_id = add_memory(&ctx, &agent, "project", "seed memory").await;
    let connected_cog = add_cognition(&ctx, &agent, "connected thought").await;
    let _unconnected_cog = add_cognition(&ctx, &agent, "unconnected thought").await;

    connect(&ctx, &Ref::memory(mem_id), &Ref::cognition(connected_cog)).await;

    let overrides = DreamOverrides {
        cognition_size: Some(100),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides).await;

    let cog_ids: HashSet<CognitionId> = context.cognitions.iter().map(|c| c.id).collect();

    assert!(
        cog_ids.contains(&connected_cog),
        "BFS should discover connected cognition"
    );
}

#[tokio::test]
async fn bfs_discovers_connected_experiences() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    let mem_id = add_memory(&ctx, &agent, "project", "seed memory").await;
    let connected_exp = add_experience(&ctx, &agent, "connected experience").await;
    let _unconnected_exp = add_experience(&ctx, &agent, "unconnected experience").await;

    connect(&ctx, &Ref::memory(mem_id), &Ref::experience(connected_exp)).await;

    let overrides = DreamOverrides {
        experience_size: Some(100),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides).await;

    let exp_ids: HashSet<ExperienceId> = context.experiences.iter().map(|e| e.id).collect();
    assert!(
        exp_ids.contains(&connected_exp),
        "BFS should discover connected experience"
    );
}

// ── Experience selection ─────────────────────────────────────────

#[tokio::test]
async fn experience_size_cap_keeps_most_recent() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    for i in 0..8 {
        add_experience(&ctx, &agent, &format!("experience {i}")).await;
    }

    let overrides = DreamOverrides {
        experience_size: Some(3),
        ..Default::default()
    };
    let context = dream_with(&ctx, &agent, &overrides).await;

    assert_eq!(context.experiences.len(), 3, "capped at experience_size");
}

// ── Connection pruning ───────────────────────────────────────────

#[tokio::test]
async fn connections_pruned_to_included_endpoints() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    let mem_id = add_memory(&ctx, &agent, "project", "seed memory").await;
    let cog_id = add_cognition(&ctx, &agent, "connected thought").await;
    let orphan_cog = add_cognition(&ctx, &agent, "orphan thought").await;

    let _included_conn = connect(&ctx, &Ref::memory(mem_id), &Ref::cognition(cog_id)).await;

    let archival_mem = add_memory(&ctx, &agent, "archival", "old memory").await;
    let _excluded_conn = connect(
        &ctx,
        &Ref::memory(archival_mem),
        &Ref::cognition(orphan_cog),
    )
    .await;

    let context = dream(&ctx, &agent).await;

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

#[tokio::test]
async fn pressures_paired_with_urge_ctas() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    for i in 0..10 {
        add_cognition(&ctx, &agent, &format!("thought {i}")).await;
    }

    let context = dream(&ctx, &agent).await;

    for reading in &context.pressures {
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

#[tokio::test]
async fn dream_overrides_change_output() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    for i in 0..10 {
        add_cognition(&ctx, &agent, &format!("thought {i}")).await;
    }
    for i in 0..10 {
        add_memory(&ctx, &agent, "project", &format!("memory {i}")).await;
    }

    let default_context = dream(&ctx, &agent).await;

    let overrides = DreamOverrides {
        cognition_size: Some(2),
        recollection_size: Some(2),
        ..Default::default()
    };
    let restricted_context = dream_with(&ctx, &agent, &overrides).await;

    assert!(
        restricted_context.cognitions.len() <= 2,
        "override should restrict cognitions"
    );
    assert!(
        restricted_context.memories.len() < default_context.memories.len()
            || default_context.memories.len() <= 3,
        "override should restrict memories"
    );
}

// ── Ordering ─────────────────────────────────────────────────────

#[tokio::test]
async fn memories_sorted_by_created_at() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    add_memory(&ctx, &agent, "core", "first core").await;
    add_memory(&ctx, &agent, "project", "first project").await;
    add_memory(&ctx, &agent, "core", "second core").await;
    add_memory(&ctx, &agent, "project", "second project").await;

    let context = dream(&ctx, &agent).await;

    for window in context.memories.windows(2) {
        assert!(
            window[0].created_at <= window[1].created_at,
            "memories should be sorted by created_at"
        );
    }
}

#[tokio::test]
async fn cognitions_sorted_by_created_at() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    for i in 0..5 {
        add_cognition(&ctx, &agent, &format!("thought {i}")).await;
    }

    let context = dream(&ctx, &agent).await;

    for window in context.cognitions.windows(2) {
        assert!(
            window[0].created_at <= window[1].created_at,
            "cognitions should be sorted by created_at"
        );
    }
}

// ── Template rendering ───────────────────────────────────────────

#[tokio::test]
async fn dream_template_renders_agent_identity() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    add_memory(&ctx, &agent, "core", "I am a thinker").await;
    add_cognition(&ctx, &agent, "something interesting").await;

    let context = dream(&ctx, &agent).await;
    let rendered = DreamTemplate::new(&context).to_string();

    assert!(
        rendered.contains("thinker.process"),
        "dream should contain agent name"
    );
    assert!(
        rendered.contains("I am a thinker"),
        "dream should contain core memory"
    );
    assert!(
        rendered.contains("something interesting"),
        "dream should contain cognition"
    );
    assert!(
        rendered.contains("## Your Identity"),
        "dream should have identity section"
    );
    assert!(
        rendered.contains("## Instructions"),
        "dream should have instructions section"
    );
}

#[tokio::test]
async fn dream_template_omits_empty_sections() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;

    let context = dream(&ctx, &agent).await;
    let rendered = DreamTemplate::new(&context).to_string();

    assert!(rendered.contains("thinker.process"));
    assert!(rendered.contains("## Cognitive Textures"));

    assert!(
        !rendered.contains("## Your Memories"),
        "empty memories section should be omitted"
    );
    assert!(
        !rendered.contains("## Your Cognitions"),
        "empty cognitions section should be omitted"
    );
    assert!(
        !rendered.contains("## Your Connections"),
        "empty connections section should be omitted"
    );
}

#[tokio::test]
async fn introspect_template_renders() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;
    let context = dream(&ctx, &agent).await;

    let pressures = RelevantPressures::from_pressures(
        context
            .pressures
            .iter()
            .map(|r| r.pressure.clone())
            .collect(),
    );
    let rendered = IntrospectTemplate::new(&context.agent, pressures).to_string();

    assert!(rendered.contains("thinker.process"));
    assert!(rendered.contains("Before your context compacts"));
}

#[tokio::test]
async fn guidebook_template_renders_vocabulary() {
    let ctx = seeded_ctx().await;
    let agent = seed_agent(&ctx).await;
    let context = dream(&ctx, &agent).await;

    let rendered = GuidebookTemplate::new(&context).to_string();

    assert!(rendered.contains("Cognitive Guidebook"));
    assert!(rendered.contains("thinker.process"));
    assert!(rendered.contains("observation"));
    assert!(rendered.contains("Your Lifecycle"));
}
