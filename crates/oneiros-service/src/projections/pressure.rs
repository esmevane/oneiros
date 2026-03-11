use oneiros_db::*;
use oneiros_model::*;

pub const ALL: &[Projection] = &[
    INTROSPECT_PRESSURE,
    CATHARSIS_PRESSURE,
    RECOLLECT_PRESSURE,
    RETROSPECT_PRESSURE,
];

const INTROSPECT_PRESSURE: Projection = Projection {
    name: "pressure:introspect",
    apply: apply_introspect_pressure,
    reset: |db| db.reset_pressures_by_urge("introspect"),
};

const CATHARSIS_PRESSURE: Projection = Projection {
    name: "pressure:catharsis",
    apply: apply_catharsis_pressure,
    reset: |db| db.reset_pressures_by_urge("catharsis"),
};

const RECOLLECT_PRESSURE: Projection = Projection {
    name: "pressure:recollect",
    apply: apply_recollect_pressure,
    reset: |db| db.reset_pressures_by_urge("recollect"),
};

const RETROSPECT_PRESSURE: Projection = Projection {
    name: "pressure:retrospect",
    apply: apply_retrospect_pressure,
    reset: |db| db.reset_pressures_by_urge("retrospect"),
};

/// Extract agent identity from an event, if the event is relevant to pressure.
///
/// Returns the resolved Agent, or None if the event is irrelevant or the agent
/// doesn't exist yet.
fn resolve_agent(db: &Database, event: &KnownEvent) -> Result<Option<Agent>, DatabaseError> {
    let agent_id = match &event.data {
        Events::Cognition(CognitionEvents::CognitionAdded(c)) => c.agent_id.to_string(),
        Events::Memory(MemoryEvents::MemoryAdded(m)) => m.agent_id.to_string(),
        Events::Experience(ExperienceEvents::ExperienceCreated(e)) => e.agent_id.to_string(),
        Events::Connection(ConnectionEvents::ConnectionCreated(_)) => {
            // Connections affect catharsis and recollect but don't carry agent_id.
            // We recompute all agents' pressures when connections change.
            return Ok(None);
        }
        Events::Introspecting(IntrospectingEvents::IntrospectionComplete(a)) => {
            match db.get_agent(&a.name)? {
                Some(agent) => return Ok(Some(agent)),
                None => return Ok(None),
            }
        }
        Events::Reflecting(ReflectingEvents::ReflectionComplete(a)) => {
            match db.get_agent(&a.name)? {
                Some(agent) => return Ok(Some(agent)),
                None => return Ok(None),
            }
        }
        Events::Lifecycle(LifecycleEvents::Woke(a)) => match db.get_agent(&a.name)? {
            Some(agent) => return Ok(Some(agent)),
            None => return Ok(None),
        },
        _ => return Ok(None),
    };

    db.get_agent_by_id(&agent_id)
}

// ── Introspect ──────────────────────────────────────────────────

/// Gather inputs from the database and construct an IntrospectGauge.
///
/// The projection's job is purely gathering — the Gauge type owns
/// the calculation logic.
fn apply_introspect_pressure(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let agent = match resolve_agent(db, event)? {
        Some(a) => a,
        None => return Ok(()),
    };
    let agent_id = agent.id.to_string();

    if db.get_urge("introspect")?.is_none() {
        return Ok(());
    }

    let last_introspect =
        db.latest_lifecycle_timestamp(agent.name.as_ref(), "introspection-complete")?;

    let hours_since_last_introspect = match &last_introspect {
        Some(ts) => hours_since(ts, &event.timestamp.to_string()),
        None => 24.0,
    };

    let total_cognitions = db.count_cognitions_for_agent(&agent_id)?;
    let working_cognitions = db.count_cognitions_by_texture_for_agent(&agent_id, "working")?;

    let since_ts = last_introspect
        .as_deref()
        .unwrap_or("1970-01-01T00:00:00.000Z");

    let cognitions_since_introspect = db.count_cognitions_since(&agent_id, since_ts)?;
    let memories_since_introspect = db.count_memories_since(&agent_id, since_ts)?;

    let last_wake = db.latest_lifecycle_timestamp(agent.name.as_ref(), "woke")?;
    let session_since = last_wake.as_deref().unwrap_or("1970-01-01T00:00:00.000Z");
    let session_cognition_count = db.count_cognitions_since(&agent_id, session_since)?;

    let inputs = IntrospectInputs {
        hours_since_last_introspect,
        total_cognitions: total_cognitions as u64,
        working_cognitions: working_cognitions as u64,
        cognitions_since_introspect: cognitions_since_introspect as u64,
        memories_since_introspect: memories_since_introspect as u64,
        session_cognition_count: session_cognition_count as u64,
    };

    let gauge = Gauge::Introspect(IntrospectGauge::from_inputs(inputs));

    let pressure = Pressure {
        id: PressureId::new(),
        agent_id: agent.id,
        urge: UrgeName::new("introspect"),
        data: gauge,
        updated_at: Timestamp::now(),
    };

    db.upsert_pressure(&pressure)?;

    Ok(())
}

// ── Catharsis ───────────────────────────────────────────────────

fn apply_catharsis_pressure(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let agent = match resolve_agent(db, event)? {
        Some(a) => a,
        None => return Ok(()),
    };
    let agent_id = agent.id.to_string();

    if db.get_urge("catharsis")?.is_none() {
        return Ok(());
    }

    let tensions_experience_count =
        db.count_experiences_by_sensation_for_agent(&agent_id, "tensions")?;

    let total_cognitions = db.count_cognitions_for_agent(&agent_id)?;
    let working_cognitions = db.count_cognitions_by_texture_for_agent(&agent_id, "working")?;

    let last_reflect = db.latest_lifecycle_timestamp(agent.name.as_ref(), "reflection-complete")?;
    let hours_since_last_reflect = match &last_reflect {
        Some(ts) => hours_since(ts, &event.timestamp.to_string()),
        None => 24.0,
    };

    let orphaned_cognitions = db.count_cognitions_not_in_experiences(&agent_id)?;

    let inputs = CatharsisInputs {
        tensions_experience_count: tensions_experience_count as u64,
        total_cognitions: total_cognitions as u64,
        working_cognitions: working_cognitions as u64,
        hours_since_last_reflect,
        orphaned_cognitions: orphaned_cognitions as u64,
    };

    let gauge = Gauge::Catharsis(CatharsisGauge::from_inputs(inputs));

    let pressure = Pressure {
        id: PressureId::new(),
        agent_id: agent.id,
        urge: UrgeName::new("catharsis"),
        data: gauge,
        updated_at: Timestamp::now(),
    };

    db.upsert_pressure(&pressure)?;

    Ok(())
}

// ── Recollect ───────────────────────────────────────────────────

fn apply_recollect_pressure(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let agent = match resolve_agent(db, event)? {
        Some(a) => a,
        None => return Ok(()),
    };
    let agent_id = agent.id.to_string();

    if db.get_urge("recollect")?.is_none() {
        return Ok(());
    }

    let session_memory_count = db.count_memories_by_level_for_agent(&agent_id, "session")?;
    let working_memory_count = db.count_memories_by_level_for_agent(&agent_id, "working")?;

    let total_experiences = db.count_experiences_for_agent(&agent_id)?;
    let unconnected_experiences = db.count_experiences_not_in_connections(&agent_id)?;

    let last_memory = db.latest_memory_timestamp_for_agent(&agent_id)?;
    let hours_since_last_memory = match &last_memory {
        Some(ts) => hours_since(ts, &event.timestamp.to_string()),
        None => 24.0,
    };

    let inputs = RecollectInputs {
        session_memory_count: session_memory_count as u64,
        total_experiences: total_experiences as u64,
        unconnected_experiences: unconnected_experiences as u64,
        hours_since_last_memory,
        working_memory_count: working_memory_count as u64,
    };

    let gauge = Gauge::Recollect(RecollectGauge::from_inputs(inputs));

    let pressure = Pressure {
        id: PressureId::new(),
        agent_id: agent.id,
        urge: UrgeName::new("recollect"),
        data: gauge,
        updated_at: Timestamp::now(),
    };

    db.upsert_pressure(&pressure)?;

    Ok(())
}

// ── Retrospect ──────────────────────────────────────────────────

fn apply_retrospect_pressure(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let agent = match resolve_agent(db, event)? {
        Some(a) => a,
        None => return Ok(()),
    };
    let agent_id = agent.id.to_string();

    if db.get_urge("retrospect")?.is_none() {
        return Ok(());
    }

    let last_archival = db.latest_memory_timestamp_by_level_for_agent(&agent_id, "archival")?;
    let hours_since_last_archival = match &last_archival {
        Some(ts) => hours_since(ts, &event.timestamp.to_string()),
        None => 168.0, // no archival ever = strong pressure (1 week)
    };

    let last_project = db.latest_memory_timestamp_by_level_for_agent(&agent_id, "project")?;
    let hours_since_last_project_memory = match &last_project {
        Some(ts) => hours_since(ts, &event.timestamp.to_string()),
        None => 48.0,
    };

    // Count wake events since last archival memory
    let archival_since = last_archival
        .as_deref()
        .unwrap_or("1970-01-01T00:00:00.000Z");
    let sessions_since_retrospect =
        db.count_lifecycle_events_since(agent.name.as_ref(), "woke", archival_since)?;

    let total_experience_count = db.count_experiences_for_agent(&agent_id)?;

    let inputs = RetrospectInputs {
        hours_since_last_archival,
        hours_since_last_project_memory,
        sessions_since_retrospect: sessions_since_retrospect as u64,
        total_experience_count: total_experience_count as u64,
    };

    let gauge = Gauge::Retrospect(RetrospectGauge::from_inputs(inputs));

    let pressure = Pressure {
        id: PressureId::new(),
        agent_id: agent.id,
        urge: UrgeName::new("retrospect"),
        data: gauge,
        updated_at: Timestamp::now(),
    };

    db.upsert_pressure(&pressure)?;

    Ok(())
}

/// Compute hours between two ISO 8601 timestamps.
/// Falls back to 0.0 on parse failure.
fn hours_since(from: &str, to: &str) -> f64 {
    let parse = |s: &str| {
        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ")
            .or_else(|_| chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%SZ"))
    };

    match (parse(from), parse(to)) {
        (Ok(a), Ok(b)) => {
            let diff = b.signed_duration_since(a);
            (diff.num_seconds() as f64 / 3600.0).max(0.0)
        }
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projections;
    use tempfile::TempDir;

    fn setup_brain() -> (TempDir, Database) {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test-pressure.db");
        let db = Database::create_brain_db(&db_path).unwrap();
        (temp, db)
    }

    fn seed_prerequisites(db: &Database, source: Source) -> Agent {
        let persona = Persona::init(
            PersonaName::new("process"),
            Description::new("Process persona"),
            Prompt::new("process prompt"),
        );
        db.log_event(
            &NewEvent::new(Events::Persona(PersonaEvents::PersonaSet(persona)), source),
            projections::BRAIN,
        )
        .unwrap();

        let texture = Texture::init(
            TextureName::new("working"),
            Description::new("Working texture"),
            Prompt::default(),
        );
        db.log_event(
            &NewEvent::new(Events::Texture(TextureEvents::TextureSet(texture)), source),
            projections::BRAIN,
        )
        .unwrap();

        let observation = Texture::init(
            TextureName::new("observation"),
            Description::new("Observation"),
            Prompt::default(),
        );
        db.log_event(
            &NewEvent::new(
                Events::Texture(TextureEvents::TextureSet(observation)),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        // Seed levels for recollect/retrospect
        for level_name in &["session", "working", "project", "archival"] {
            let level = Level::init(
                LevelName::new(level_name),
                Description::new(format!("{level_name} level")),
                Prompt::default(),
            );
            db.log_event(
                &NewEvent::new(Events::Level(LevelEvents::LevelSet(level)), source),
                projections::BRAIN,
            )
            .unwrap();
        }

        // Seed sensations for catharsis
        let tensions = Sensation::init(
            SensationName::new("tensions"),
            Description::new("Productive friction"),
            Prompt::default(),
        );
        db.log_event(
            &NewEvent::new(
                Events::Sensation(SensationEvents::SensationSet(tensions)),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        // Seed natures for recollect (connections need a nature)
        let context_nature = Nature::init(
            NatureName::new("context"),
            Description::new("Context relationship"),
            Prompt::default(),
        );
        db.log_event(
            &NewEvent::new(
                Events::Nature(NatureEvents::NatureSet(context_nature)),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        // Seed all four urges — required for FK constraints
        for urge_name in &["introspect", "catharsis", "recollect", "retrospect"] {
            let urge = Urge::init(
                UrgeName::new(urge_name),
                Description::new(format!("{urge_name} urge")),
                Prompt::new(format!("{urge_name} prompt")),
            );
            db.log_event(
                &NewEvent::new(Events::Urge(UrgeEvents::UrgeSet(urge)), source),
                projections::BRAIN,
            )
            .unwrap();
        }

        let agent = Agent::init(
            "test agent",
            "test prompt",
            AgentName::new("test.agent"),
            PersonaName::new("process"),
        );
        db.log_event(
            &NewEvent::new(
                Events::Agent(AgentEvents::AgentCreated(agent.clone())),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        agent
    }

    fn wake_agent(db: &Database, agent: &Agent, source: Source) {
        db.log_event(
            &NewEvent::new(
                Events::Lifecycle(LifecycleEvents::Woke(SelectAgentByName {
                    name: agent.name.clone(),
                })),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();
    }

    fn find_pressure<'a>(pressures: &'a [Pressure], urge: &str) -> Option<&'a Pressure> {
        pressures.iter().find(|p| p.urge.as_ref() == urge)
    }

    // ── Introspect tests ────────────────────────────────────────

    #[test]
    fn no_cognitions_produces_baseline_urgency() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        wake_agent(&db, &agent, source);

        let pressures = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let pressure = find_pressure(&pressures, "introspect").expect("introspect pressure");

        let urgency = pressure.urgency();
        assert!(urgency > 0.0, "should have some urgency from time factor");
        assert!(urgency < 1.0, "should be less than 1.0");

        let Gauge::Introspect(ref g) = pressure.data else {
            panic!("expected introspect gauge");
        };
        assert_eq!(g.inputs.total_cognitions, 0);
        assert_eq!(g.inputs.session_cognition_count, 0);
        assert!(g.inputs.hours_since_last_introspect > 0.0);
    }

    #[test]
    fn cognitions_increase_introspect_pressure() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        wake_agent(&db, &agent, source);

        let pressures_before = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let urgency_before = find_pressure(&pressures_before, "introspect")
            .unwrap()
            .urgency();

        for i in 0..5 {
            let cognition = Cognition::create(
                agent.id,
                TextureName::new("working"),
                Content::new(format!("working thought {i}")),
            );
            db.log_event(
                &NewEvent::new(
                    Events::Cognition(CognitionEvents::CognitionAdded(cognition)),
                    source,
                ),
                projections::BRAIN,
            )
            .unwrap();
        }

        let pressures_after = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let urgency_after = find_pressure(&pressures_after, "introspect")
            .unwrap()
            .urgency();

        assert!(
            urgency_after > urgency_before,
            "introspect urgency should increase after adding working cognitions \
             (before: {urgency_before}, after: {urgency_after})"
        );
    }

    // ── Catharsis tests ─────────────────────────────────────────

    #[test]
    fn catharsis_baseline() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        wake_agent(&db, &agent, source);

        let pressures = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let pressure = find_pressure(&pressures, "catharsis").expect("catharsis pressure");

        let Gauge::Catharsis(ref g) = pressure.data else {
            panic!("expected catharsis gauge");
        };
        assert_eq!(g.inputs.tensions_experience_count, 0);
        // Time factor should be > 0 (no reflect ever = 24h default)
        assert!(g.inputs.hours_since_last_reflect > 0.0);
    }

    #[test]
    fn tensions_increase_catharsis_pressure() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        wake_agent(&db, &agent, source);

        let pressures_before = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let urgency_before = find_pressure(&pressures_before, "catharsis")
            .unwrap()
            .urgency();

        // Add tensions experiences
        for i in 0..3 {
            let experience = Experience::create(
                agent.id,
                SensationName::new("tensions"),
                Description::new(format!("tension {i}")),
            );
            db.log_event(
                &NewEvent::new(
                    Events::Experience(ExperienceEvents::ExperienceCreated(experience)),
                    source,
                ),
                projections::BRAIN,
            )
            .unwrap();
        }

        let pressures_after = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let urgency_after = find_pressure(&pressures_after, "catharsis")
            .unwrap()
            .urgency();

        assert!(
            urgency_after > urgency_before,
            "catharsis urgency should increase after adding tensions \
             (before: {urgency_before}, after: {urgency_after})"
        );
    }

    // ── Recollect tests ─────────────────────────────────────────

    #[test]
    fn recollect_baseline() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        wake_agent(&db, &agent, source);

        let pressures = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let pressure = find_pressure(&pressures, "recollect").expect("recollect pressure");

        let Gauge::Recollect(ref g) = pressure.data else {
            panic!("expected recollect gauge");
        };
        assert_eq!(g.inputs.session_memory_count, 0);
        assert_eq!(g.inputs.total_experiences, 0);
    }

    #[test]
    fn memories_update_recollect_gauge_inputs() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        wake_agent(&db, &agent, source);

        // Add session-level memories
        for i in 0..5 {
            let memory = Memory::create(
                agent.id,
                LevelName::new("session"),
                Content::new(format!("session knowledge {i}")),
            );
            db.log_event(
                &NewEvent::new(Events::Memory(MemoryEvents::MemoryAdded(memory)), source),
                projections::BRAIN,
            )
            .unwrap();
        }

        let pressures = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let pressure = find_pressure(&pressures, "recollect").unwrap();

        let Gauge::Recollect(ref g) = pressure.data else {
            panic!("expected recollect gauge");
        };

        // Session memories should be counted in inputs
        assert_eq!(g.inputs.session_memory_count, 5);
        // Adding memories is responding to the urge — time resets, reducing pressure
        assert!(g.inputs.hours_since_last_memory < 1.0);
    }

    #[test]
    fn unconnected_experiences_increase_recollect_pressure() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        wake_agent(&db, &agent, source);

        let pressures_before = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let urgency_before = find_pressure(&pressures_before, "recollect")
            .unwrap()
            .urgency();

        // Add experiences without connecting them
        for i in 0..5 {
            let experience = Experience::create(
                agent.id,
                SensationName::new("tensions"),
                Description::new(format!("isolated insight {i}")),
            );
            db.log_event(
                &NewEvent::new(
                    Events::Experience(ExperienceEvents::ExperienceCreated(experience)),
                    source,
                ),
                projections::BRAIN,
            )
            .unwrap();
        }

        let pressures_after = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let urgency_after = find_pressure(&pressures_after, "recollect")
            .unwrap()
            .urgency();

        assert!(
            urgency_after > urgency_before,
            "recollect urgency should increase with unconnected experiences \
             (before: {urgency_before}, after: {urgency_after})"
        );
    }

    // ── Retrospect tests ────────────────────────────────────────

    #[test]
    fn retrospect_baseline() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        wake_agent(&db, &agent, source);

        let pressures = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let pressure = find_pressure(&pressures, "retrospect").expect("retrospect pressure");

        let Gauge::Retrospect(ref g) = pressure.data else {
            panic!("expected retrospect gauge");
        };
        // No archival ever = strong default pressure
        assert!(g.inputs.hours_since_last_archival > 0.0);
        assert!(pressure.urgency() > 0.0);
    }

    // ── All four pressures ──────────────────────────────────────

    #[test]
    fn wake_produces_all_four_pressures() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        wake_agent(&db, &agent, source);

        let pressures = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();

        assert_eq!(
            pressures.len(),
            4,
            "should have 4 pressures, got: {:?}",
            pressures
                .iter()
                .map(|p| p.urge.as_ref())
                .collect::<Vec<_>>()
        );

        assert!(find_pressure(&pressures, "introspect").is_some());
        assert!(find_pressure(&pressures, "catharsis").is_some());
        assert!(find_pressure(&pressures, "recollect").is_some());
        assert!(find_pressure(&pressures, "retrospect").is_some());
    }

    #[test]
    fn replay_preserves_all_pressures() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        wake_agent(&db, &agent, source);

        for i in 0..3 {
            let cognition = Cognition::create(
                agent.id,
                TextureName::new("working"),
                Content::new(format!("thought {i}")),
            );
            db.log_event(
                &NewEvent::new(
                    Events::Cognition(CognitionEvents::CognitionAdded(cognition)),
                    source,
                ),
                projections::BRAIN,
            )
            .unwrap();
        }

        let pressures_before = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();

        db.replay(projections::BRAIN).unwrap();

        let pressures_after = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();

        assert_eq!(pressures_before.len(), pressures_after.len());

        for urge in &["introspect", "catharsis", "recollect", "retrospect"] {
            let before = find_pressure(&pressures_before, urge).unwrap().urgency();
            let after = find_pressure(&pressures_after, urge).unwrap().urgency();
            assert!(
                (before - after).abs() < 0.01,
                "replay should produce same {urge} urgency (before: {before}, after: {after})"
            );
        }
    }

    // ── Utility tests ───────────────────────────────────────────

    #[test]
    fn hours_since_same_time() {
        let ts = "2026-03-10T12:00:00.000Z";
        assert!((hours_since(ts, ts) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn hours_since_four_hours() {
        let from = "2026-03-10T08:00:00.000Z";
        let to = "2026-03-10T12:00:00.000Z";
        assert!((hours_since(from, to) - 4.0).abs() < 0.01);
    }

    #[test]
    fn hours_since_invalid_returns_zero() {
        assert!((hours_since("not-a-date", "also-not") - 0.0).abs() < f64::EPSILON);
    }
}
