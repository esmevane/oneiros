use std::collections::HashSet;

use crate::tests::harness::{Retryable, TestApp};
use crate::*;

async fn seeded_context() -> (ProjectLog, TestApp) {
    let app = TestApp::new()
        .await
        .expect("boot test app")
        .init_system()
        .await
        .expect("init system")
        .init_project()
        .await
        .expect("init project")
        .seed_core()
        .await
        .expect("seed core");

    let context = app.project_log();
    (context, app)
}

async fn seed_agent(context: &ProjectLog) -> AgentName {
    AgentService::create(
        context,
        &CreateAgent::V1(
            CreateAgentV1::builder()
                .name("thinker")
                .persona("process")
                .description("A thinking agent")
                .prompt("You think")
                .build(),
        ),
    )
    .await
    .unwrap();
    AgentName::new("thinker.process")
}

async fn add_cognition(context: &ProjectLog, agent: &AgentName, content: &str) -> CognitionId {
    match CognitionService::add(
        context,
        &AddCognition::builder_v1()
            .agent(agent.clone())
            .texture("observation")
            .content(content)
            .build()
            .into(),
    )
    .await
    .unwrap()
    {
        CognitionResponse::CognitionAdded(CognitionAddedResponse::V1(added)) => added.cognition.id,
        other => panic!("expected CognitionAdded, got {other:?}"),
    }
}

async fn add_memory(
    context: &ProjectLog,
    agent: &AgentName,
    level: &str,
    content: &str,
) -> MemoryId {
    match MemoryService::add(
        context,
        &AddMemory::builder_v1()
            .agent(agent.clone())
            .level(level)
            .content(content)
            .build()
            .into(),
    )
    .await
    .unwrap()
    {
        MemoryResponse::MemoryAdded(MemoryAddedResponse::V1(added)) => added.memory.id,
        other => panic!("expected MemoryAdded, got {other:?}"),
    }
}

async fn add_experience(
    context: &ProjectLog,
    agent: &AgentName,
    description: &str,
) -> ExperienceId {
    match ExperienceService::create(
        context,
        &CreateExperience::builder_v1()
            .agent(agent.clone())
            .sensation("echoes")
            .description(description)
            .build()
            .into(),
    )
    .await
    .unwrap()
    {
        ExperienceResponse::ExperienceCreated(ExperienceCreatedResponse::V1(created)) => {
            created.experience.id
        }
        other => panic!("expected ExperienceCreated, got {other:?}"),
    }
}

async fn connect(context: &ProjectLog, from: &Ref, to: &Ref) -> ConnectionId {
    match ConnectionService::create(
        context,
        &CreateConnection::builder_v1()
            .from_ref(RefToken::from(from.clone()))
            .to_ref(RefToken::from(to.clone()))
            .nature("reference")
            .build()
            .into(),
    )
    .await
    .unwrap()
    {
        ConnectionResponse::ConnectionCreated(ConnectionCreatedResponse::V1(created)) => {
            created.connection.id
        }
        other => panic!("expected ConnectionCreated, got {other:?}"),
    }
}

async fn dream(context: &ProjectLog, agent: &AgentName) -> DreamContext {
    dream_with(context, agent, &DreamOverrides::default()).await
}

/// Retry the dream and a check closure together until both succeed.
/// Use this when writes seeded after the agent need to be visible in
/// the dream — the projector applies them eventually, so a single dream
/// may succeed before all related data has propagated.
async fn dream_satisfying<F>(context: &ProjectLog, agent: &AgentName, label: &str, check: F)
where
    F: FnMut(&DreamContext) -> Result<(), String>,
{
    dream_with_satisfying(context, agent, &DreamOverrides::default(), label, check).await
}

async fn dream_with_satisfying<F>(
    context: &ProjectLog,
    agent: &AgentName,
    overrides: &DreamOverrides,
    label: &str,
    mut check: F,
) where
    F: FnMut(&DreamContext) -> Result<(), String>,
{
    let interval = std::time::Duration::from_millis(16);
    let timeout = std::time::Duration::from_secs(2);
    let deadline = std::time::Instant::now() + timeout;
    let mut last_failure = String::from("never evaluated");

    loop {
        let response = ContinuityService::dream(
            context,
            &DreamAgent::builder_v1().agent(agent.clone()).build().into(),
            overrides,
        )
        .await;

        let attempt = match response {
            Ok(ContinuityResponse::Dreaming(DreamingResponse::V1(details))) => {
                check(&details.context)
            }
            Ok(other) => Err(format!("expected Dreaming, got {other:?}")),
            Err(err) => Err(format!("{err:?}")),
        };

        match attempt {
            Ok(()) => return,
            Err(msg) => {
                last_failure = msg;
                if std::time::Instant::now() >= deadline {
                    panic!("{label}: not met within {timeout:?}.\nLast failure: {last_failure}");
                }
                tokio::time::sleep(interval).await;
            }
        }
    }
}

async fn dream_with(
    context: &ProjectLog,
    agent: &AgentName,
    overrides: &DreamOverrides,
) -> DreamContext {
    // Projections apply asynchronously; the dream call reads through
    // them. Retry until the agent is visible and the dream resolves.
    Retryable::default()
        .wait_for_async(
            || async {
                ContinuityService::dream(
                    context,
                    &DreamAgent::builder_v1().agent(agent.clone()).build().into(),
                    overrides,
                )
                .await
                .map_err(|e| format!("{e:?}"))
                .and_then(|response| match response {
                    ContinuityResponse::Dreaming(DreamingResponse::V1(details)) => {
                        Ok(details.context)
                    }
                    other => Err(format!("expected Dreaming, got {other:?}")),
                })
            },
            "dream context available",
        )
        .await
}

// ── Vocabulary ───────────────────────────────────────────────────

#[tokio::test]
async fn dream_includes_all_vocabulary() {
    let (context, _app) = seeded_context().await;
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
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let context = dream(&context, &agent).await;

    let persona = context.persona.expect("persona should be present");
    assert_eq!(persona.name, PersonaName::new("process"));
}

// ── Memory filtering ─────────────────────────────────────────────

#[tokio::test]
async fn core_memories_always_included() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_memory(&context, &agent, "core", "identity fundament").await;
    add_memory(&context, &agent, "archival", "old history").await;

    dream_satisfying(&context, &agent, "core memories visible", |dream| {
        let contents: Vec<&str> = dream.memories.iter().map(|m| m.content.as_str()).collect();
        if !contents.contains(&"identity fundament") {
            return Err("core memory missing".into());
        }
        if contents.contains(&"old history") {
            return Err("archival should be excluded at project threshold".into());
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn level_threshold_filters_lower_priority() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_memory(&context, &agent, "core", "core memory").await;
    add_memory(&context, &agent, "project", "project memory").await;
    add_memory(&context, &agent, "session", "session memory").await;
    add_memory(&context, &agent, "working", "working memory").await;
    add_memory(&context, &agent, "archival", "archival memory").await;

    dream_satisfying(&context, &agent, "level threshold applied", |dream| {
        let levels: Vec<&str> = dream.memories.iter().map(|m| m.level.as_str()).collect();
        for required in ["core", "project", "session", "working"] {
            if !levels.contains(&required) {
                return Err(format!("missing level {required}"));
            }
        }
        if levels.contains(&"archival") {
            return Err("archival should be below project threshold".into());
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn level_threshold_override_changes_filter() {
    let (context, _app) = seeded_context().await;
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

    dream_with_satisfying(
        &context,
        &agent,
        &overrides,
        "core-only override applied",
        |dream| {
            let levels: Vec<&str> = dream.memories.iter().map(|m| m.level.as_str()).collect();
            if !levels.contains(&"core") {
                return Err("core should always be included".into());
            }
            if levels.len() != 1 {
                return Err(format!("expected only core, got {levels:?}"));
            }
            Ok(())
        },
    )
    .await;
}

#[tokio::test]
async fn recollection_size_caps_non_core_memories() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..5 {
        add_memory(&context, &agent, "project", &format!("project memory {i}")).await;
    }
    add_memory(&context, &agent, "core", "core memory").await;

    let overrides = DreamOverrides {
        recollection_size: Some(2),
        ..Default::default()
    };

    dream_with_satisfying(
        &context,
        &agent,
        &overrides,
        "recollection size cap applied",
        |dream| {
            let core_count = dream
                .memories
                .iter()
                .filter(|m| m.level.as_str() == "core")
                .count();
            let non_core_count = dream
                .memories
                .iter()
                .filter(|m| m.level.as_str() != "core")
                .count();
            if core_count != 1 {
                return Err(format!("expected 1 core, got {core_count}"));
            }
            if non_core_count != 2 {
                return Err(format!("expected non_core=2, got {non_core_count}"));
            }
            Ok(())
        },
    )
    .await;
}

// ── Cognition selection ──────────────────────────────────────────

#[tokio::test]
async fn sparse_graph_includes_all_cognitions() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..5 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }

    dream_satisfying(&context, &agent, "all cognitions visible", |dream| {
        if dream.cognitions.len() != 5 {
            return Err(format!("expected 5 cognitions, got {}", dream.cognitions.len()));
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn cognition_size_cap_keeps_most_recent() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..10 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }

    let overrides = DreamOverrides {
        cognition_size: Some(3),
        ..Default::default()
    };

    dream_with_satisfying(
        &context,
        &agent,
        &overrides,
        "cognition size cap applied",
        |dream| {
            if dream.cognitions.len() != 3 {
                return Err(format!("expected 3 cognitions, got {}", dream.cognitions.len()));
            }
            let contents: Vec<&str> =
                dream.cognitions.iter().map(|c| c.content.as_str()).collect();
            for required in ["thought 9", "thought 8", "thought 7"] {
                if !contents.contains(&required) {
                    return Err(format!("missing recent cognition {required}"));
                }
            }
            Ok(())
        },
    )
    .await;
}

// ── BFS graph traversal ──────────────────────────────────────────

#[tokio::test]
async fn bfs_discovers_connected_cognitions() {
    let (context, _app) = seeded_context().await;
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

    dream_with_satisfying(
        &context,
        &agent,
        &overrides,
        "BFS discovers connected cognition",
        |dream| {
            let cog_ids: HashSet<CognitionId> = dream.cognitions.iter().map(|c| c.id).collect();
            if !cog_ids.contains(&connected_cog) {
                return Err("connected cognition not discovered".into());
            }
            Ok(())
        },
    )
    .await;
}

#[tokio::test]
async fn bfs_discovers_connected_experiences() {
    let (context, _app) = seeded_context().await;
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

    dream_with_satisfying(
        &context,
        &agent,
        &overrides,
        "BFS discovers connected experience",
        |dream| {
            let exp_ids: HashSet<ExperienceId> = dream.experiences.iter().map(|e| e.id).collect();
            if !exp_ids.contains(&connected_exp) {
                return Err("connected experience not discovered".into());
            }
            Ok(())
        },
    )
    .await;
}

// ── Experience selection ─────────────────────────────────────────

#[tokio::test]
async fn experience_size_cap_keeps_most_recent() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..8 {
        add_experience(&context, &agent, &format!("experience {i}")).await;
    }

    let overrides = DreamOverrides {
        experience_size: Some(3),
        ..Default::default()
    };

    dream_with_satisfying(
        &context,
        &agent,
        &overrides,
        "experience size cap applied",
        |dream| {
            if dream.experiences.len() != 3 {
                return Err(format!(
                    "expected 3 experiences, got {}",
                    dream.experiences.len()
                ));
            }
            Ok(())
        },
    )
    .await;
}

// ── Connection pruning ───────────────────────────────────────────

#[tokio::test]
async fn connections_pruned_to_included_endpoints() {
    let (context, _app) = seeded_context().await;
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

    dream_satisfying(&context, &agent, "connections pruned to endpoints", |dream| {
        // Wait for the included connection to be visible — once both
        // sides of the graph have caught up, the pruning invariant
        // should hold for every connection in the dream.
        if !dream
            .connections
            .iter()
            .any(|c| c.from_ref == Ref::memory(mem_id) && c.to_ref == Ref::cognition(cog_id))
        {
            return Err("included connection not yet visible".into());
        }
        let included_refs: HashSet<Ref> = dream
            .memories
            .iter()
            .map(|m| Ref::memory(m.id))
            .chain(dream.cognitions.iter().map(|c| Ref::cognition(c.id)))
            .chain(dream.experiences.iter().map(|e| Ref::experience(e.id)))
            .collect();
        for conn in &dream.connections {
            if !included_refs.contains(&conn.from_ref) {
                return Err(format!("from_ref {:?} not in included", conn.from_ref));
            }
            if !included_refs.contains(&conn.to_ref) {
                return Err(format!("to_ref {:?} not in included", conn.to_ref));
            }
        }
        Ok(())
    })
    .await;
}

// ── Pressure readings ────────────────────────────────────────────

#[tokio::test]
async fn pressures_paired_with_urge_ctas() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..10 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }

    dream_satisfying(&context, &agent, "pressures paired with urges", |dream| {
        if dream.pressures.is_empty() {
            return Err("pressures not yet populated".into());
        }
        for reading in &dream.pressures {
            if let Some(urge) = dream.urges.iter().find(|u| u.name == reading.pressure.urge) {
                if reading.cta != urge.prompt {
                    return Err(format!(
                        "cta mismatch: {:?} != {:?}",
                        reading.cta, urge.prompt
                    ));
                }
            }
        }
        Ok(())
    })
    .await;
}

// ── Config override integration ──────────────────────────────────

#[tokio::test]
async fn dream_overrides_change_output() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..10 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }
    for i in 0..10 {
        add_memory(&context, &agent, "project", &format!("memory {i}")).await;
    }

    // Wait for the default dream to have all 10 cognitions before
    // comparing — otherwise the assertions race the projector.
    let mut default_cognition_count = 0usize;
    let mut default_memory_count = 0usize;
    dream_satisfying(&context, &agent, "default dream populated", |dream| {
        if dream.cognitions.len() < 10 {
            return Err(format!(
                "only {} cognitions visible, want 10",
                dream.cognitions.len()
            ));
        }
        default_cognition_count = dream.cognitions.len();
        default_memory_count = dream.memories.len();
        Ok(())
    })
    .await;

    let overrides = DreamOverrides {
        cognition_size: Some(2),
        recollection_size: Some(2),
        ..Default::default()
    };

    dream_with_satisfying(
        &context,
        &agent,
        &overrides,
        "overrides restrict output",
        |dream| {
            if dream.cognitions.len() > 2 {
                return Err(format!(
                    "override should cap cognitions at 2, got {}",
                    dream.cognitions.len()
                ));
            }
            if dream.memories.len() >= default_memory_count && default_memory_count > 3 {
                return Err(format!(
                    "override should restrict memories below default {default_memory_count}, got {}",
                    dream.memories.len()
                ));
            }
            Ok(())
        },
    )
    .await;
}

// ── Ordering ─────────────────────────────────────────────────────

#[tokio::test]
async fn memories_sorted_by_created_at() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_memory(&context, &agent, "core", "first core").await;
    add_memory(&context, &agent, "project", "first project").await;
    add_memory(&context, &agent, "core", "second core").await;
    add_memory(&context, &agent, "project", "second project").await;

    dream_satisfying(&context, &agent, "memories sorted by created_at", |dream| {
        if dream.memories.len() < 4 {
            return Err(format!(
                "want >=4 memories, got {}",
                dream.memories.len()
            ));
        }
        for window in dream.memories.windows(2) {
            if window[0].created_at > window[1].created_at {
                return Err("memories not sorted by created_at".into());
            }
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn cognitions_sorted_by_created_at() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..5 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }

    dream_satisfying(&context, &agent, "cognitions sorted by created_at", |dream| {
        if dream.cognitions.len() < 5 {
            return Err(format!(
                "want >=5 cognitions, got {}",
                dream.cognitions.len()
            ));
        }
        for window in dream.cognitions.windows(2) {
            if window[0].created_at > window[1].created_at {
                return Err("cognitions not sorted by created_at".into());
            }
        }
        Ok(())
    })
    .await;
}

// ── Greeting rendering ──────────────────────────────────────────

#[tokio::test]
async fn greeting_opens_with_identity_and_date() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let context = dream(&context, &agent).await;
    let rendered = DreamTemplate::new(&context).to_string();

    assert!(
        rendered.starts_with("You are waking as thinker.process. Today is "),
        "greeting must open with identity sentence and date, got: {}",
        &rendered[..rendered.len().min(120)]
    );
}

#[tokio::test]
async fn greeting_includes_core_memories_inline() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_memory(&context, &agent, "core", "I am fundamentally a thinker").await;

    dream_satisfying(&context, &agent, "core memory inline in greeting", |dream| {
        let rendered = DreamTemplate::new(dream).to_string();
        if !rendered.contains("### Your core memories") {
            return Err("missing core memories block".into());
        }
        if !rendered.contains("I am fundamentally a thinker") {
            return Err("core memory content not inlined".into());
        }
        if !rendered.contains("ref:") {
            return Err("core memories not addressable by ref".into());
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn greeting_omits_non_core_memories() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_memory(&context, &agent, "core", "core identity").await;
    add_memory(
        &context,
        &agent,
        "project",
        "A very long project memory that should not appear in the greeting",
    )
    .await;

    dream_satisfying(&context, &agent, "non-core memories omitted", |dream| {
        let rendered = DreamTemplate::new(dream).to_string();
        if !rendered.contains("core identity") {
            return Err("core memory missing from greeting".into());
        }
        if rendered.contains("should not appear in the greeting") {
            return Err("non-core memory leaked into greeting".into());
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn greeting_shows_latest_cognitions_with_refs() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_cognition(&context, &agent, "first thought").await;
    add_cognition(&context, &agent, "second thought").await;

    dream_satisfying(&context, &agent, "latest cognitions in greeting", |dream| {
        let rendered = DreamTemplate::new(dream).to_string();
        if !rendered.contains("### Latest cognitions") {
            return Err("missing latest cognitions block".into());
        }
        if !rendered.contains("second thought") {
            return Err("most recent cognition not visible".into());
        }
        if !rendered.contains("oneiros cognition list") {
            return Err("missing cognition list hint".into());
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn greeting_caps_latest_cognitions_to_three() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..6 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }

    dream_satisfying(&context, &agent, "greeting caps cognitions to three", |dream| {
        let rendered = DreamTemplate::new(dream).to_string();
        for required in ["thought 5", "thought 4", "thought 3"] {
            if !rendered.contains(required) {
                return Err(format!("missing {required}"));
            }
        }
        for forbidden in ["thought 0", "thought 1"] {
            if rendered.contains(forbidden) {
                return Err(format!("older cognition {forbidden} should not appear"));
            }
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn greeting_shows_latest_experiences_with_refs() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    add_experience(&context, &agent, "first moment").await;

    dream_satisfying(&context, &agent, "latest experiences in greeting", |dream| {
        let rendered = DreamTemplate::new(dream).to_string();
        if !rendered.contains("### Latest experiences") {
            return Err("missing latest experiences block".into());
        }
        if !rendered.contains("first moment") {
            return Err("experience content not visible".into());
        }
        if !rendered.contains("oneiros experience list") {
            return Err("missing experience list hint".into());
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn greeting_shows_latest_threads_as_connections() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let mem = add_memory(&context, &agent, "core", "anchor").await;
    let cog = add_cognition(&context, &agent, "linked thought").await;
    connect(&context, &Ref::memory(mem), &Ref::cognition(cog)).await;

    dream_satisfying(&context, &agent, "latest threads visible", |dream| {
        let rendered = DreamTemplate::new(dream).to_string();
        if !rendered.contains("### Latest threads") {
            return Err("missing latest threads block".into());
        }
        if !rendered.contains("[reference]") {
            return Err("thread line missing connection nature".into());
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn greeting_omits_vocabulary_sections() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let context = dream(&context, &agent).await;
    let rendered = DreamTemplate::new(&context).to_string();

    assert!(
        !rendered.contains("## Cognitive Textures"),
        "vocabulary belongs in the guidebook, not the greeting"
    );
    assert!(!rendered.contains("## Memory Levels"));
    assert!(!rendered.contains("## Sensations"));
    assert!(!rendered.contains("## Natures"));
    assert!(!rendered.contains("## Urges"));
}

#[tokio::test]
async fn greeting_pressure_omits_verbose_forms() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    for i in 0..10 {
        add_cognition(&context, &agent, &format!("thought {i}")).await;
    }

    dream_satisfying(&context, &agent, "compact pressure gauge", |dream| {
        // Wait for cognitions to flow through so pressures populate.
        if dream.cognitions.is_empty() {
            return Err("cognitions not yet visible".into());
        }
        let rendered = DreamTemplate::new(dream).to_string();
        if rendered.contains("tensions:") {
            return Err("verbose 'tensions:' should be absent".into());
        }
        if rendered.contains("orphaned:") {
            return Err("verbose 'orphaned:' should be absent".into());
        }
        if rendered.contains("## Pressure Gauge") {
            return Err("verbose pressure section should be absent".into());
        }
        if !dream.pressures.is_empty() && !rendered.contains("[urges:") {
            return Err("non-empty pressures should render as compact gauge".into());
        }
        Ok(())
    })
    .await;
}

#[tokio::test]
async fn greeting_includes_next_steps_and_hints() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let context = dream(&context, &agent).await;
    let rendered = DreamTemplate::new(&context).to_string();

    assert!(
        rendered.contains("## Next steps"),
        "greeting should include next steps section"
    );
    assert!(
        rendered.contains("morning pages"),
        "next steps should point at the morning pages practice"
    );

    assert!(
        rendered.contains("## Hints"),
        "greeting should include hints section"
    );
    assert!(
        rendered.contains("oneiros search"),
        "hints should point at search"
    );
    assert!(
        rendered.contains("oneiros guidebook thinker.process"),
        "hints should point at the agent's guidebook"
    );
}

#[tokio::test]
async fn greeting_omits_legacy_sections() {
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;

    let context = dream(&context, &agent).await;
    let rendered = DreamTemplate::new(&context).to_string();

    assert!(
        !rendered.contains("## Your Identity"),
        "legacy identity section should be gone"
    );
    assert!(
        !rendered.contains("## Your Persona"),
        "legacy persona section should be gone"
    );
    assert!(
        !rendered.contains("## Agent Definition"),
        "legacy agent definition pointer should be gone"
    );
    assert!(
        !rendered.contains("## Instructions"),
        "legacy instructions section should be gone"
    );
    assert!(
        !rendered.contains("### Morning Pages\n"),
        "morning pages now lives in the skill, not the dream"
    );
}

#[tokio::test]
async fn introspect_template_renders() {
    let (context, _app) = seeded_context().await;
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
    let (context, _app) = seeded_context().await;
    let agent = seed_agent(&context).await;
    let context = dream(&context, &agent).await;

    let rendered = GuidebookTemplate::new(&context).to_string();

    assert!(rendered.contains("Cognitive Guidebook"));
    assert!(rendered.contains("thinker.process"));
    assert!(rendered.contains("observation"));
    assert!(rendered.contains("Your Lifecycle"));
}
