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
    reset: |db| db.reset_expressions(),
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
    reset: |_| Ok(()),
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
    reset: |_| Ok(()),
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
    reset: |_| Ok(()),
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
    reset: |_| Ok(()),
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
