use std::collections::HashSet;

use crate::tests::harness::TestApp;
use crate::*;

async fn seeded_app() -> Result<TestApp, Error> {
    TestApp::new()
        .await?
        .init_system()
        .await?
        .init_project()
        .await?
        .seed_core()
        .await
}

async fn with_agent(app: &TestApp) -> Result<AgentName, Error> {
    app.command("emerge thinker process --description 'A thinking agent'")
        .await?;
    Ok(AgentName::new("thinker.process"))
}

fn extract_dream(rendered: Rendered<Responses>) -> DreamContext {
    match rendered.into_response() {
        Responses::Continuity(ContinuityResponse::Dreaming(ctx)) => ctx,
        other => panic!("expected Dreaming, got {other:?}"),
    }
}

fn extract_cognition_id(rendered: Rendered<Responses>) -> CognitionId {
    match rendered.into_response() {
        Responses::Cognition(CognitionResponse::CognitionAdded(c)) => c.data.id,
        other => panic!("expected CognitionAdded, got {other:?}"),
    }
}

fn extract_memory_id(rendered: Rendered<Responses>) -> MemoryId {
    match rendered.into_response() {
        Responses::Memory(MemoryResponse::MemoryAdded(m)) => m.data.id,
        other => panic!("expected MemoryAdded, got {other:?}"),
    }
}

fn extract_experience_id(rendered: Rendered<Responses>) -> ExperienceId {
    match rendered.into_response() {
        Responses::Experience(ExperienceResponse::ExperienceCreated(e)) => e.data.id,
        other => panic!("expected ExperienceCreated, got {other:?}"),
    }
}

fn extract_connection_id(rendered: Rendered<Responses>) -> ConnectionId {
    match rendered.into_response() {
        Responses::Connection(ConnectionResponse::ConnectionCreated(c)) => c.data.id,
        other => panic!("expected ConnectionCreated, got {other:?}"),
    }
}

async fn add_cognition(app: &TestApp, agent: &AgentName, content: &str) -> Result<CognitionId, Error> {
    let rendered = app
        .command(&format!(r#"cognition add {agent} observation "{content}""#))
        .await?;
    Ok(extract_cognition_id(rendered))
}

async fn add_memory(app: &TestApp, agent: &AgentName, level: &str, content: &str) -> Result<MemoryId, Error> {
    let rendered = app
        .command(&format!(r#"memory add {agent} {level} "{content}""#))
        .await?;
    Ok(extract_memory_id(rendered))
}

async fn add_experience(app: &TestApp, agent: &AgentName, description: &str) -> Result<ExperienceId, Error> {
    let rendered = app
        .command(&format!(r#"experience create {agent} echoes "{description}""#))
        .await?;
    Ok(extract_experience_id(rendered))
}

async fn connect(app: &TestApp, from: &RefToken, to: &RefToken) -> Result<ConnectionId, Error> {
    let rendered = app
        .command(&format!("connection create reference {from} {to}"))
        .await?;
    Ok(extract_connection_id(rendered))
}

async fn dream(app: &TestApp, agent: &AgentName) -> Result<DreamContext, Error> {
    let rendered = app.command(&format!("dream {agent}")).await?;
    Ok(extract_dream(rendered))
}

// ── Vocabulary ───────────────────────────────────────────────────

#[tokio::test]
async fn dream_includes_all_vocabulary() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    let context = dream(&app, &agent).await.unwrap();

    assert_eq!(context.textures.len(), 10, "seed creates 10 textures");
    assert_eq!(context.levels.len(), 5, "seed creates 5 levels");
    assert_eq!(context.sensations.len(), 6, "seed creates 6 sensations");
    assert_eq!(context.natures.len(), 6, "seed creates 6 natures");
    assert_eq!(context.urges.len(), 4, "seed creates 4 urges");
}

#[tokio::test]
async fn dream_includes_persona() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    let context = dream(&app, &agent).await.unwrap();

    let persona = context.persona.expect("persona should be present");
    assert_eq!(persona.name, PersonaName::new("process"));
}

// ── Memory filtering ─────────────────────────────────────────────

#[tokio::test]
async fn core_memories_always_included() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    add_memory(&app, &agent, "core", "identity fundament").await.unwrap();
    add_memory(&app, &agent, "archival", "old history").await.unwrap();

    let context = dream(&app, &agent).await.unwrap();

    let memory_contents: Vec<&str> = context.memories.iter().map(|m| m.content.as_str()).collect();
    assert!(memory_contents.contains(&"identity fundament"), "core memories must always be included");
    assert!(!memory_contents.contains(&"old history"), "archival memories should be excluded at project level threshold");
}

#[tokio::test]
async fn level_threshold_filters_lower_priority() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    add_memory(&app, &agent, "core", "core memory").await.unwrap();
    add_memory(&app, &agent, "project", "project memory").await.unwrap();
    add_memory(&app, &agent, "session", "session memory").await.unwrap();
    add_memory(&app, &agent, "working", "working memory").await.unwrap();
    add_memory(&app, &agent, "archival", "archival memory").await.unwrap();

    let context = dream(&app, &agent).await.unwrap();

    let levels: Vec<&str> = context.memories.iter().map(|m| m.level.as_str()).collect();
    assert!(levels.contains(&"core"), "core always included");
    assert!(levels.contains(&"project"), "project >= threshold");
    assert!(levels.contains(&"session"), "session >= threshold");
    assert!(levels.contains(&"working"), "working >= threshold");
    assert!(!levels.contains(&"archival"), "archival below project threshold");
}

#[tokio::test]
async fn recollection_size_caps_non_core_memories() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    for i in 0..5 {
        add_memory(&app, &agent, "project", &format!("project memory {i}")).await.unwrap();
    }
    add_memory(&app, &agent, "core", "core memory").await.unwrap();

    let context = dream(&app, &agent).await.unwrap();

    let core_count = context.memories.iter().filter(|m| m.level.as_str() == "core").count();
    let non_core_count = context.memories.iter().filter(|m| m.level.as_str() != "core").count();

    assert_eq!(core_count, 1, "core memory is always included");
    // Default recollection_size is 30, so all 5 should be included
    assert_eq!(non_core_count, 5, "all project memories within default cap");
}

// ── Cognition selection ──────────────────────────────────────────

#[tokio::test]
async fn sparse_graph_includes_all_cognitions() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    for i in 0..5 {
        add_cognition(&app, &agent, &format!("thought {i}")).await.unwrap();
    }

    let context = dream(&app, &agent).await.unwrap();
    assert_eq!(context.cognitions.len(), 5, "sparse graph should include all cognitions");
}

// ── BFS graph traversal ──────────────────────────────────────────

#[tokio::test]
async fn bfs_discovers_connected_cognitions() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    let mem_id = add_memory(&app, &agent, "project", "seed memory").await.unwrap();
    let connected_cog = add_cognition(&app, &agent, "connected thought").await.unwrap();
    let _unconnected_cog = add_cognition(&app, &agent, "unconnected thought").await.unwrap();

    connect(
        &app,
        &RefToken::new(Ref::memory(mem_id)),
        &RefToken::new(Ref::cognition(connected_cog)),
    )
    .await
    .unwrap();

    let context = dream(&app, &agent).await.unwrap();

    let cog_ids: HashSet<CognitionId> = context.cognitions.iter().map(|c| c.id).collect();
    assert!(cog_ids.contains(&connected_cog), "BFS should discover connected cognition");
}

#[tokio::test]
async fn bfs_discovers_connected_experiences() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    let mem_id = add_memory(&app, &agent, "project", "seed memory").await.unwrap();
    let connected_exp = add_experience(&app, &agent, "connected experience").await.unwrap();
    let _unconnected_exp = add_experience(&app, &agent, "unconnected experience").await.unwrap();

    connect(
        &app,
        &RefToken::new(Ref::memory(mem_id)),
        &RefToken::new(Ref::experience(connected_exp)),
    )
    .await
    .unwrap();

    let context = dream(&app, &agent).await.unwrap();

    let exp_ids: HashSet<ExperienceId> = context.experiences.iter().map(|e| e.id).collect();
    assert!(exp_ids.contains(&connected_exp), "BFS should discover connected experience");
}

// ── Experience selection ─────────────────────────────────────────

#[tokio::test]
async fn experience_size_cap_keeps_most_recent() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    for i in 0..8 {
        add_experience(&app, &agent, &format!("experience {i}")).await.unwrap();
    }

    let context = dream(&app, &agent).await.unwrap();

    assert!(
        context.experiences.len() <= 8,
        "experience count should be capped by default experience_size"
    );
    assert!(
        !context.experiences.is_empty(),
        "should have at least some experiences"
    );
}

// ── Connection pruning ───────────────────────────────────────────

#[tokio::test]
async fn connections_pruned_to_included_endpoints() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    let mem_id = add_memory(&app, &agent, "project", "seed memory").await.unwrap();
    let cog_id = add_cognition(&app, &agent, "connected thought").await.unwrap();
    let orphan_cog = add_cognition(&app, &agent, "orphan thought").await.unwrap();

    let _included_conn = connect(
        &app,
        &RefToken::new(Ref::memory(mem_id)),
        &RefToken::new(Ref::cognition(cog_id)),
    )
    .await
    .unwrap();

    let archival_mem = add_memory(&app, &agent, "archival", "old memory").await.unwrap();
    let _excluded_conn = connect(
        &app,
        &RefToken::new(Ref::memory(archival_mem)),
        &RefToken::new(Ref::cognition(orphan_cog)),
    )
    .await
    .unwrap();

    let context = dream(&app, &agent).await.unwrap();

    let included_refs: HashSet<Ref> = context
        .memories.iter().map(|m| Ref::memory(m.id))
        .chain(context.cognitions.iter().map(|c| Ref::cognition(c.id)))
        .chain(context.experiences.iter().map(|e| Ref::experience(e.id)))
        .collect();

    for conn in &context.connections {
        assert!(included_refs.contains(&conn.from_ref), "connection from_ref {:?} should be in included entities", conn.from_ref);
        assert!(included_refs.contains(&conn.to_ref), "connection to_ref {:?} should be in included entities", conn.to_ref);
    }
}

// ── Pressure readings ────────────────────────────────────────────

#[tokio::test]
async fn pressures_paired_with_urge_ctas() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    for i in 0..10 {
        add_cognition(&app, &agent, &format!("thought {i}")).await.unwrap();
    }

    let context = dream(&app, &agent).await.unwrap();

    for reading in &context.pressures {
        let urge = context.urges.iter().find(|u| u.name == reading.pressure.urge);
        if let Some(urge) = urge {
            assert_eq!(reading.cta, urge.prompt, "pressure CTA should match urge prompt");
        }
    }
}

// ── Ordering ─────────────────────────────────────────────────────

#[tokio::test]
async fn memories_sorted_by_created_at() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    add_memory(&app, &agent, "core", "first core").await.unwrap();
    add_memory(&app, &agent, "project", "first project").await.unwrap();
    add_memory(&app, &agent, "core", "second core").await.unwrap();
    add_memory(&app, &agent, "project", "second project").await.unwrap();

    let context = dream(&app, &agent).await.unwrap();

    for window in context.memories.windows(2) {
        assert!(window[0].created_at <= window[1].created_at, "memories should be sorted by created_at");
    }
}

#[tokio::test]
async fn cognitions_sorted_by_created_at() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    for i in 0..5 {
        add_cognition(&app, &agent, &format!("thought {i}")).await.unwrap();
    }

    let context = dream(&app, &agent).await.unwrap();

    for window in context.cognitions.windows(2) {
        assert!(window[0].created_at <= window[1].created_at, "cognitions should be sorted by created_at");
    }
}

// ── Compact rendering (default) ─────────────────────────────────

#[tokio::test]
async fn compact_dream_vocabulary_shows_names_only() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    app.command("texture set observation --description 'Noticing things' --prompt 'When you notice something interesting about the code, architecture, or process, capture it as an observation. Focus on what you see, not what to do about it.'")
        .await.unwrap();

    let rendered = app.command(&format!("dream {agent}")).await.unwrap();
    let prompt = rendered.prompt().to_string();

    assert!(prompt.contains("observation"), "compact dream should list texture names");
    assert!(!prompt.contains("Focus on what you see, not what to do about it"), "compact dream should not include full texture prompts");
    assert!(prompt.contains("guidebook"), "compact dream should reference the guidebook for full vocabulary");
}

#[tokio::test]
async fn compact_dream_core_memories_inline() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    add_memory(&app, &agent, "core", "I am fundamentally a thinker").await.unwrap();

    let rendered = app.command(&format!("dream {agent}")).await.unwrap();
    let prompt = rendered.prompt().to_string();

    assert!(prompt.contains("I am fundamentally a thinker"), "core memories should appear inline in compact dream");
}

#[tokio::test]
async fn compact_dream_non_core_memories_as_summary() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    add_memory(&app, &agent, "core", "core identity").await.unwrap();
    add_memory(&app, &agent, "project", "A very long project memory that contains detailed architectural information about the system design and implementation patterns that were discovered during session work").await.unwrap();

    let rendered = app.command(&format!("dream {agent}")).await.unwrap();
    let prompt = rendered.prompt().to_string();

    assert!(prompt.contains("core identity"), "core memory should be fully inline");
    assert!(!prompt.contains("that were discovered during session work"), "non-core memory should be truncated in compact dream");
    assert!(prompt.contains("ref:"), "compact dream should include ref tokens for summarized memories");
}

// ── Deep rendering ──────────────────────────────────────────────

#[tokio::test]
async fn deep_dream_vocabulary_shows_full_prompts() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    app.command("texture set observation --description 'Noticing things' --prompt 'When you notice something interesting about the code, architecture, or process, capture it as an observation. Focus on what you see, not what to do about it.'")
        .await.unwrap();

    let rendered = app.command(&format!("dream {agent} --deep")).await.unwrap();
    let prompt = rendered.prompt().to_string();

    assert!(prompt.contains("observation"), "deep dream should list texture names");
    assert!(prompt.contains("Focus on what you see, not what to do about it"), "deep dream should include full texture prompts");
}

#[tokio::test]
async fn deep_dream_all_memories_inline() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    add_memory(&app, &agent, "core", "core identity").await.unwrap();
    add_memory(&app, &agent, "project", "A very long project memory that contains detailed architectural information about the system design and implementation patterns that were discovered during session work").await.unwrap();

    let rendered = app.command(&format!("dream {agent} --deep")).await.unwrap();
    let prompt = rendered.prompt().to_string();

    assert!(prompt.contains("core identity"), "core memory should be inline in deep dream");
    assert!(prompt.contains("that were discovered during session work"), "non-core memory should be fully inline in deep dream");
}

// ── Template rendering ───────────────────────────────────────────

#[tokio::test]
async fn dream_template_renders_agent_identity() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    add_memory(&app, &agent, "core", "I am a thinker").await.unwrap();
    add_cognition(&app, &agent, "something interesting").await.unwrap();

    let rendered = app.command(&format!("dream {agent}")).await.unwrap();
    let prompt = rendered.prompt().to_string();

    assert!(prompt.contains("thinker.process"), "dream should contain agent name");
    assert!(prompt.contains("I am a thinker"), "dream should contain core memory");
    assert!(prompt.contains("something interesting"), "dream should contain cognition");
    assert!(prompt.contains("## Your Identity"), "dream should have identity section");
    assert!(prompt.contains("## Instructions"), "dream should have instructions section");
}

#[tokio::test]
async fn dream_template_omits_empty_sections() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    let rendered = app.command(&format!("dream {agent}")).await.unwrap();
    let prompt = rendered.prompt().to_string();

    assert!(prompt.contains("thinker.process"));
    assert!(prompt.contains("## Cognitive Textures"));
    assert!(!prompt.contains("## Your Memories"), "empty memories section should be omitted");
    assert!(!prompt.contains("## Your Cognitions"), "empty cognitions section should be omitted");
    assert!(!prompt.contains("## Your Connections"), "empty connections section should be omitted");
}

#[tokio::test]
async fn introspect_template_renders() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    let rendered = app.command(&format!("introspect {agent}")).await.unwrap();
    let prompt = rendered.prompt().to_string();

    assert!(prompt.contains("thinker.process"));
    assert!(prompt.contains("Before your context compacts"));
}

#[tokio::test]
async fn guidebook_template_renders_vocabulary() {
    let app = seeded_app().await.unwrap();
    let agent = with_agent(&app).await.unwrap();

    let rendered = app.command(&format!("guidebook {agent}")).await.unwrap();
    let prompt = rendered.prompt().to_string();

    assert!(prompt.contains("Cognitive Guidebook"));
    assert!(prompt.contains("thinker.process"));
    assert!(prompt.contains("observation"));
    assert!(prompt.contains("Your Lifecycle"));
}
