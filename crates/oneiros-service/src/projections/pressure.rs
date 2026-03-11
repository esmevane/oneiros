use oneiros_db::*;
use oneiros_model::*;

pub const ALL: &[Projection] = &[INTROSPECT_PRESSURE];

const INTROSPECT_PRESSURE: Projection = Projection {
    name: "pressure:introspect",
    apply: apply_introspect_pressure,
    reset: |db| db.reset_pressures_by_urge("introspect"),
};

/// Gather inputs from the database and construct an IntrospectGauge.
///
/// The projection's job is purely gathering — the Gauge type owns
/// the calculation logic.
fn apply_introspect_pressure(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    // Only recompute on events that affect the factors.
    let agent_id = match &event.data {
        Events::Cognition(CognitionEvents::CognitionAdded(c)) => c.agent_id.to_string(),
        Events::Memory(MemoryEvents::MemoryAdded(m)) => m.agent_id.to_string(),
        Events::Introspecting(IntrospectingEvents::IntrospectionComplete(a)) => {
            match db.get_agent(&a.name)? {
                Some(agent) => agent.id.to_string(),
                None => return Ok(()),
            }
        }
        Events::Lifecycle(LifecycleEvents::Woke(a)) => match db.get_agent(&a.name)? {
            Some(agent) => agent.id.to_string(),
            None => return Ok(()),
        },
        _ => return Ok(()),
    };

    let agent = match db.get_agent_by_id(&agent_id)? {
        Some(a) => a,
        None => return Ok(()),
    };

    // Gather inputs for the introspect heuristic.
    let last_introspect =
        db.latest_lifecycle_timestamp(agent.name.as_ref(), "introspection-complete")?;

    let hours_since_last_introspect = match &last_introspect {
        Some(ts) => hours_since(ts, &event.timestamp.to_string()),
        None => 24.0, // no introspection ever = strong pressure
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

    // Verify the urge exists — if not seeded yet, skip gracefully.
    if db.get_urge("introspect")?.is_none() {
        return Ok(());
    }

    // Construct the gauge — all calculation happens in the model.
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

        let level = Level::init(
            LevelName::new("session"),
            Description::new("Session level"),
            Prompt::default(),
        );
        db.log_event(
            &NewEvent::new(Events::Level(LevelEvents::LevelSet(level)), source),
            projections::BRAIN,
        )
        .unwrap();

        // Seed the introspect urge — required for FK constraint on pressure table
        let urge = Urge::init(
            UrgeName::new("introspect"),
            Description::new("Cognitive consolidation"),
            Prompt::new("introspect prompt"),
        );
        db.log_event(
            &NewEvent::new(Events::Urge(UrgeEvents::UrgeSet(urge)), source),
            projections::BRAIN,
        )
        .unwrap();

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

    #[test]
    fn no_cognitions_produces_baseline_urgency() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        // Wake the agent to trigger pressure computation
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

        let pressures = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();

        assert_eq!(pressures.len(), 1);
        let pressure = &pressures[0];
        assert_eq!(pressure.urge.as_ref(), "introspect");

        // Urgency computed from gauge, not a stored column
        let urgency = pressure.urgency();
        assert!(urgency > 0.0, "should have some urgency from time factor");
        assert!(urgency < 1.0, "should be less than 1.0");

        // Verify the gauge carries inputs and calculation
        let Gauge::Introspect(ref g) = pressure.data;
        assert_eq!(g.inputs.total_cognitions, 0);
        assert_eq!(g.inputs.session_cognition_count, 0);
        assert!(g.inputs.hours_since_last_introspect > 0.0);
    }

    #[test]
    fn cognitions_increase_pressure() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        // Wake
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

        let pressures_before = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();
        let urgency_before = pressures_before[0].urgency();

        // Add several working cognitions
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
        let urgency_after = pressures_after[0].urgency();

        assert!(
            urgency_after > urgency_before,
            "urgency should increase after adding working cognitions \
             (before: {urgency_before}, after: {urgency_after})"
        );
    }

    #[test]
    fn replay_preserves_pressure() {
        let (_temp, db) = setup_brain();
        let source = Source::default();
        let agent = seed_prerequisites(&db, source);

        // Wake and add some cognitions
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

        // Replay
        db.replay(projections::BRAIN).unwrap();

        let pressures_after = db.list_pressures_for_agent(&agent.id.to_string()).unwrap();

        assert_eq!(pressures_before.len(), pressures_after.len());
        assert!(
            (pressures_before[0].urgency() - pressures_after[0].urgency()).abs() < 0.01,
            "replay should produce same urgency (before: {}, after: {})",
            pressures_before[0].urgency(),
            pressures_after[0].urgency(),
        );
    }

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
