use oneiros_db::*;
use oneiros_model::*;
use serde_json::Value;

/// Search projections emit expressions into the FTS5 index.
///
/// These run alongside brain projections — same events, different target table.
/// The expressions table + FTS5 virtual table enable full-text search across
/// the cognitive stream.
pub const ALL: &[Projection] = &[
    COGNITION_ADDED,
    MEMORY_ADDED,
    EXPERIENCE_CREATED,
    EXPERIENCE_DESCRIPTION_UPDATED,
    AGENT_CREATED,
    AGENT_UPDATED,
    AGENT_REMOVED,
    PERSONA_SET,
    PERSONA_REMOVED,
];

// -- Cognition ----------------------------------------------------------------

const COGNITION_ADDED: Projection = Projection {
    name: "search:cognition-added",
    events: &["cognition-added"],
    apply: apply_cognition_added,
    reset: |db| db.reset_expressions_by_kind("cognition-content"),
};

fn apply_cognition_added(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let cognition: Cognition = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::cognition(cognition.id);

    db.insert_expression(
        &resource_ref,
        "cognition-content",
        cognition.content.as_str(),
    )?;

    Ok(())
}

// -- Memory -------------------------------------------------------------------

const MEMORY_ADDED: Projection = Projection {
    name: "search:memory-added",
    events: &["memory-added"],
    apply: apply_memory_added,
    reset: |db| db.reset_expressions_by_kind("memory-content"),
};

fn apply_memory_added(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let memory: Memory = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::memory(memory.id);

    db.insert_expression(&resource_ref, "memory-content", memory.content.as_str())?;

    Ok(())
}

// -- Experience ---------------------------------------------------------------

const EXPERIENCE_CREATED: Projection = Projection {
    name: "search:experience-created",
    events: &["experience-created"],
    apply: apply_experience_created,
    reset: |db| db.reset_expressions_by_kind("experience-description"),
};

fn apply_experience_created(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let experience: Experience = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::experience(experience.id);

    db.insert_expression(
        &resource_ref,
        "experience-description",
        experience.description.as_str(),
    )?;

    Ok(())
}

const EXPERIENCE_DESCRIPTION_UPDATED: Projection = Projection {
    name: "search:experience-description-updated",
    events: &["experience-description-updated"],
    apply: apply_experience_description_updated,
    reset: |_| Ok(()),
};

#[derive(serde::Deserialize)]
struct DescriptionUpdated {
    experience_id: ExperienceId,
    description: Description,
}

fn apply_experience_description_updated(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let updated: DescriptionUpdated = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::experience(updated.experience_id);

    db.delete_expressions_by_ref(&resource_ref)?;
    db.insert_expression(
        &resource_ref,
        "experience-description",
        updated.description.as_str(),
    )?;

    Ok(())
}

// -- Agent --------------------------------------------------------------------

const AGENT_CREATED: Projection = Projection {
    name: "search:agent-created",
    events: &["agent-created"],
    apply: apply_agent_created,
    reset: |db| {
        db.reset_expressions_by_kind("agent-description")?;
        db.reset_expressions_by_kind("agent-prompt")
    },
};

fn apply_agent_created(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let agent: Agent = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::agent(agent.id);

    db.insert_expression(
        &resource_ref,
        "agent-description",
        agent.description.as_str(),
    )?;
    db.insert_expression(&resource_ref, "agent-prompt", agent.prompt.as_str())?;

    Ok(())
}

const AGENT_UPDATED: Projection = Projection {
    name: "search:agent-updated",
    events: &["agent-updated"],
    apply: apply_agent_updated,
    reset: |_| Ok(()),
};

fn apply_agent_updated(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let agent: Agent = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::agent(agent.id);

    db.delete_expressions_by_ref(&resource_ref)?;
    db.insert_expression(
        &resource_ref,
        "agent-description",
        agent.description.as_str(),
    )?;
    db.insert_expression(&resource_ref, "agent-prompt", agent.prompt.as_str())?;

    Ok(())
}

const AGENT_REMOVED: Projection = Projection {
    name: "search:agent-removed",
    events: &["agent-removed"],
    apply: apply_agent_removed,
    reset: |_| Ok(()),
};

#[derive(serde::Deserialize)]
struct AgentRemoved {
    name: AgentName,
}

fn apply_agent_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: AgentRemoved = serde_json::from_value(data.clone())?;

    if let Some(agent) = db.get_agent(&removed.name)? {
        let resource_ref = Ref::agent(agent.id);
        db.delete_expressions_by_ref(&resource_ref)?;
    }

    Ok(())
}

// -- Persona ------------------------------------------------------------------

const PERSONA_SET: Projection = Projection {
    name: "search:persona-set",
    events: &["persona-set"],
    apply: apply_persona_set,
    reset: |db| {
        db.reset_expressions_by_kind("persona-description")?;
        db.reset_expressions_by_kind("persona-prompt")
    },
};

fn apply_persona_set(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let persona: Persona = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::persona(persona.name.clone());

    db.delete_expressions_by_ref(&resource_ref)?;
    db.insert_expression(
        &resource_ref,
        "persona-description",
        persona.description.as_str(),
    )?;
    db.insert_expression(&resource_ref, "persona-prompt", persona.prompt.as_str())?;

    Ok(())
}

const PERSONA_REMOVED: Projection = Projection {
    name: "search:persona-removed",
    events: &["persona-removed"],
    apply: apply_persona_removed,
    reset: |_| Ok(()),
};

#[derive(serde::Deserialize)]
struct PersonaRemoved {
    name: PersonaName,
}

fn apply_persona_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: PersonaRemoved = serde_json::from_value(data.clone())?;
    let resource_ref = Ref::persona(removed.name);

    db.delete_expressions_by_ref(&resource_ref)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::projections;
    use oneiros_db::Database;
    use oneiros_model::*;
    use tempfile::TempDir;

    fn setup_brain() -> (TempDir, Database) {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test-brain.db");
        let db = Database::create_brain_db(&db_path).unwrap();
        (temp, db)
    }

    #[test]
    fn replay_produces_same_expression_count() {
        let (_temp, db) = setup_brain();
        let source = Source::default();

        // Seed prerequisites
        let persona = Persona::init(
            PersonaName::new("test-persona"),
            Description::new("A test persona"),
            Prompt::new("test persona prompt"),
        );
        db.log_event(
            &Event::create(Events::Persona(PersonaEvents::PersonaSet(persona)), source),
            projections::BRAIN,
        )
        .unwrap();

        let texture = Texture::init(
            TextureName::new("observation"),
            Description::new("Observations"),
            Prompt::default(),
        );
        db.log_event(
            &Event::create(Events::Texture(TextureEvents::TextureSet(texture)), source),
            projections::BRAIN,
        )
        .unwrap();

        let level = Level::init(
            LevelName::new("session"),
            Description::new("Session-level"),
            Prompt::default(),
        );
        db.log_event(
            &Event::create(Events::Level(LevelEvents::LevelSet(level)), source),
            projections::BRAIN,
        )
        .unwrap();

        let sensation = Sensation::init(
            SensationName::new("caused"),
            Description::new("Causal connection"),
            Prompt::default(),
        );
        db.log_event(
            &Event::create(
                Events::Sensation(SensationEvents::SensationSet(sensation)),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        // Seed entities
        let agent = Agent::init(
            "test agent description",
            "test agent prompt",
            AgentName::new("test-agent"),
            PersonaName::new("test-persona"),
        );
        db.log_event(
            &Event::create(
                Events::Agent(AgentEvents::AgentCreated(agent.clone())),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        let cognition = Cognition::create(
            agent.id,
            TextureName::new("observation"),
            Content::new("an interesting observation about architecture"),
        );
        db.log_event(
            &Event::create(
                Events::Cognition(CognitionEvents::CognitionAdded(cognition)),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        let memory = Memory::create(
            agent.id,
            LevelName::new("session"),
            Content::new("a consolidated memory about patterns"),
        );
        db.log_event(
            &Event::create(Events::Memory(MemoryEvents::MemoryAdded(memory)), source),
            projections::BRAIN,
        )
        .unwrap();

        let experience = Experience::create(
            agent.id,
            SensationName::new("caused"),
            Description::new("one thought produced another"),
        );
        db.log_event(
            &Event::create(
                Events::Experience(ExperienceEvents::ExperienceCreated(experience)),
                source,
            ),
            projections::BRAIN,
        )
        .unwrap();

        // Count expressions after initial seeding
        let count_before = db.count_expressions().unwrap();
        assert!(count_before > 0, "should have expressions after seeding");

        // Replay all events — resets then re-applies
        db.replay(projections::BRAIN).unwrap();

        // Count again — must be identical
        let count_after = db.count_expressions().unwrap();
        assert_eq!(
            count_before, count_after,
            "replay should produce identical expression count"
        );
    }
}
