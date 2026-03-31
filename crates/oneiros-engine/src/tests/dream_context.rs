use std::collections::HashSet;

use crate::*;

async fn seeded_context() -> (ProjectContext, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("create tempdir");
    let config = Config::builder()
        .data_dir(dir.path().to_path_buf())
        .brain(BrainName::new("test"))
        .build();

    config.bootstrap().expect("bootstrap");

    let system = config.system();
    SystemService::init(&system, &InitSystem::builder().name("test").build())
        .await
        .unwrap();

    ProjectService::init(
        &system,
        &InitProject::builder().name(BrainName::new("test")).build(),
    )
    .await
    .unwrap();

    let context = config.project();
    SeedService::core(&context).await.expect("seed core");
    (context, dir)
}

async fn seed_agent(context: &ProjectContext) -> AgentName {
    AgentService::create(
        context,
        &CreateAgent::builder()
            .name("thinker")
            .persona("process")
            .description("A thinking agent")
            .prompt("You think")
            .build(),
    )
    .await
    .unwrap();
    AgentName::new("thinker.process")
}

async fn add_cognition(context: &ProjectContext, agent: &AgentName, content: &str) -> CognitionId {
    match CognitionService::add(
        context,
        &AddCognition::builder()
            .agent(agent.clone())
            .texture("observation")
            .content(content)
            .build(),
    )
    .await
    .unwrap()
    {
        CognitionResponse::CognitionAdded(c) => c.id,
        other => panic!("expected CognitionAdded, got {other:?}"),
    }
}

async fn add_memory(
    context: &ProjectContext,
    agent: &AgentName,
    level: &str,
    content: &str,
) -> MemoryId {
    match MemoryService::add(
        context,
        &AddMemory::builder()
            .agent(agent.clone())
            .level(level)
            .content(content)
            .build(),
    )
    .await
    .unwrap()
    {
        MemoryResponse::MemoryAdded(m) => m.id,
        other => panic!("expected MemoryAdded, got {other:?}"),
    }
}

async fn add_experience(
    context: &ProjectContext,
    agent: &AgentName,
    description: &str,
) -> ExperienceId {
    match ExperienceService::create(
        context,
        &CreateExperience::builder()
            .agent(agent.clone())
            .sensation("echoes")
            .description(description)
            .build(),
    )
    .await
    .unwrap()
    {
        ExperienceResponse::ExperienceCreated(e) => e.id,
        other => panic!("expected ExperienceCreated, got {other:?}"),
    }
}

async fn connect(context: &ProjectContext, from: &Ref, to: &Ref) -> ConnectionId {
    match ConnectionService::create(
        context,
        &CreateConnection::builder()
            .from_ref(RefToken::from(from.clone()))
            .to_ref(RefToken::from(to.clone()))
            .nature("reference")
            .build(),
    )
    .await
    .unwrap()
    {
        ConnectionResponse::ConnectionCreated(c) => c.id,
        other => panic!("expected ConnectionCreated, got {other:?}"),
    }
}

async fn dream(context: &ProjectContext, agent: &AgentName) -> DreamContext {
    dream_with(context, agent, &DreamOverrides::default()).await
}

async fn dream_with(
    context: &ProjectContext,
    agent: &AgentName,
    overrides: &DreamOverrides,
) -> DreamContext {
    match ContinuityService::dream(
        context,
        &DreamAgent::builder().agent(agent.clone()).build(),
        overrides,
    )
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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let context = dream(&context, &agent).await;

    assert_eq!(context.textures.len(), 10, "seed creates 10 textures");
    assert_eq!(context.levels.len(), 5, "seed creates 5 levels");
    assert_eq!(context.sensations.len(), 6, "seed creates 6 sensations");
    assert_eq!(context.natures.len(), 6, "seed creates 6 natures");
    assert_eq!(context.urges.len(), 4, "seed creates 4 urges");
}

#[tokio::test]
async fn dream_includes_persona() {
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let context = dream(&context, &agent).await;

    let persona = context.persona.expect("persona should be present");
    assert_eq!(persona.name, PersonaName::new("process"));
}

// ── Memory filtering ─────────────────────────────────────────────

#[tokio::test]
async fn core_memories_always_included() {
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_memory(&context, &agent, "core", "identity fundament").await;
    add_memory(&context, &agent, "archival", "old history").await;

    let context = dream(&context, &agent).await;

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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_memory(&context, &agent, "core", "core memory").await;
    add_memory(&context, &agent, "project", "project memory").await;
    add_memory(&context, &agent, "session", "session memory").await;
    add_memory(&context, &agent, "working", "working memory").await;
    add_memory(&context, &agent, "archival", "archival memory").await;

    let context = dream(&context, &agent).await;

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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_memory(&context, &agent, "core", "core memory").await;
    add_memory(&context, &agent, "project", "project memory").await;
    add_memory(&context, &agent, "session", "session memory").await;
    add_memory(&context, &agent, "working", "working memory").await;
    add_memory(&context, &agent, "archival", "archival memory").await;

    let overrides = DreamOverrides {
        recollection_level: Some(LevelName::new("core")),
        ..Default::default()
    };
    let context = dream_with(&context, &agent, &overrides).await;

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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..5 {
        add_memory(&context, &agent, "project", &format!("project memory {i}")).await;
    }
    add_memory(&context, &agent, "core", "core memory").await;

    let overrides = DreamOverrides {
        recollection_size: Some(2),
        ..Default::default()
    };
    let context = dream_with(&context, &agent, &overrides).await;

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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..5 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }

    let context = dream(&context, &agent).await;
    assert_eq!(
        context.cognitions.len(),
        5,
        "sparse graph should include all cognitions"
    );
}

#[tokio::test]
async fn cognition_size_cap_keeps_most_recent() {
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..10 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }

    let overrides = DreamOverrides {
        cognition_size: Some(3),
        ..Default::default()
    };
    let context = dream_with(&context, &agent, &overrides).await;

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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let mem_id = add_memory(&context, &agent, "project", "seed memory").await;
    let connected_cog = add_cognition(&context, &agent, "connected thought").await;
    let _unconnected_cog = add_cognition(&context, &agent, "unconnected thought").await;

    connect(
        &context,
        &Ref::memory(mem_id),
        &Ref::cognition(connected_cog),
    )
    .await;

    let overrides = DreamOverrides {
        cognition_size: Some(100),
        ..Default::default()
    };
    let context = dream_with(&context, &agent, &overrides).await;

    let cog_ids: HashSet<CognitionId> = context.cognitions.iter().map(|c| c.id).collect();

    assert!(
        cog_ids.contains(&connected_cog),
        "BFS should discover connected cognition"
    );
}

#[tokio::test]
async fn bfs_discovers_connected_experiences() {
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let mem_id = add_memory(&context, &agent, "project", "seed memory").await;
    let connected_exp = add_experience(&context, &agent, "connected experience").await;
    let _unconnected_exp = add_experience(&context, &agent, "unconnected experience").await;

    connect(
        &context,
        &Ref::memory(mem_id),
        &Ref::experience(connected_exp),
    )
    .await;

    let overrides = DreamOverrides {
        experience_size: Some(100),
        ..Default::default()
    };
    let context = dream_with(&context, &agent, &overrides).await;

    let exp_ids: HashSet<ExperienceId> = context.experiences.iter().map(|e| e.id).collect();
    assert!(
        exp_ids.contains(&connected_exp),
        "BFS should discover connected experience"
    );
}

// ── Experience selection ─────────────────────────────────────────

#[tokio::test]
async fn experience_size_cap_keeps_most_recent() {
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..8 {
        add_experience(&context, &agent, &format!("experience {i}")).await;
    }

    let overrides = DreamOverrides {
        experience_size: Some(3),
        ..Default::default()
    };
    let context = dream_with(&context, &agent, &overrides).await;

    assert_eq!(context.experiences.len(), 3, "capped at experience_size");
}

// ── Connection pruning ───────────────────────────────────────────

#[tokio::test]
async fn connections_pruned_to_included_endpoints() {
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let mem_id = add_memory(&context, &agent, "project", "seed memory").await;
    let cog_id = add_cognition(&context, &agent, "connected thought").await;
    let orphan_cog = add_cognition(&context, &agent, "orphan thought").await;

    let _included_conn = connect(&context, &Ref::memory(mem_id), &Ref::cognition(cog_id)).await;

    let archival_mem = add_memory(&context, &agent, "archival", "old memory").await;
    let _excluded_conn = connect(
        &context,
        &Ref::memory(archival_mem),
        &Ref::cognition(orphan_cog),
    )
    .await;

    let context = dream(&context, &agent).await;

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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..10 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }

    let context = dream(&context, &agent).await;

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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..10 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }
    for i in 0..10 {
        add_memory(&context, &agent, "project", &format!("memory {i}")).await;
    }

    let default_context = dream(&context, &agent).await;

    let overrides = DreamOverrides {
        cognition_size: Some(2),
        recollection_size: Some(2),
        ..Default::default()
    };
    let restricted_context = dream_with(&context, &agent, &overrides).await;

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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_memory(&context, &agent, "core", "first core").await;
    add_memory(&context, &agent, "project", "first project").await;
    add_memory(&context, &agent, "core", "second core").await;
    add_memory(&context, &agent, "project", "second project").await;

    let context = dream(&context, &agent).await;

    for window in context.memories.windows(2) {
        assert!(
            window[0].created_at <= window[1].created_at,
            "memories should be sorted by created_at"
        );
    }
}

#[tokio::test]
async fn cognitions_sorted_by_created_at() {
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..5 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }

    let context = dream(&context, &agent).await;

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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_memory(&context, &agent, "core", "I am a thinker").await;
    add_cognition(&context, &agent, "something interesting").await;

    let context = dream(&context, &agent).await;
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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let context = dream(&context, &agent).await;
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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;
    let context = dream(&context, &agent).await;

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
    let (context, _dir) = seeded_context().await;
    let agent = seed_agent(&context).await;
    let context = dream(&context, &agent).await;

    let rendered = GuidebookTemplate::new(&context).to_string();

    assert!(rendered.contains("Cognitive Guidebook"));
    assert!(rendered.contains("thinker.process"));
    assert!(rendered.contains("observation"));
    assert!(rendered.contains("Your Lifecycle"));
}
