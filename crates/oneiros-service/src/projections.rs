use oneiros_db::{Database, DatabaseError, Projection};
use oneiros_model::{
    Actor, Agent, AgentName, Brain, Cognition, Content, Experience, ExperienceId, Level, LevelName,
    Memory, Persona, PersonaName, RecordRef, Sensation, SensationName, StorageEntry, StorageKey,
    Tenant, Texture, TextureName, Ticket,
};
use serde_json::Value;

/// System projections, ordered by dependency (tenant before actor, actor before brain,
/// brain before ticket).
pub const SYSTEM_PROJECTIONS: &[Projection] = &[
    TENANT_PROJECTION,
    ACTOR_PROJECTION,
    BRAIN_PROJECTION,
    TICKET_ISSUED_PROJECTION,
];

/// Brain-level projections for data that lives within each brain's database.
///
/// Ordering: persona before agent (agent has FK to persona),
/// sensation before experience (experience has FK to sensation).
pub const BRAIN_PROJECTIONS: &[Projection] = &[
    LEVEL_SET_PROJECTION,
    LEVEL_REMOVED_PROJECTION,
    PERSONA_SET_PROJECTION,
    PERSONA_REMOVED_PROJECTION,
    TEXTURE_SET_PROJECTION,
    TEXTURE_REMOVED_PROJECTION,
    SENSATION_SET_PROJECTION,
    SENSATION_REMOVED_PROJECTION,
    AGENT_CREATED_PROJECTION,
    AGENT_UPDATED_PROJECTION,
    AGENT_REMOVED_PROJECTION,
    COGNITION_ADDED_PROJECTION,
    MEMORY_ADDED_PROJECTION,
    EXPERIENCE_CREATED_PROJECTION,
    EXPERIENCE_REF_ADDED_PROJECTION,
    EXPERIENCE_DESCRIPTION_UPDATED_PROJECTION,
    STORAGE_SET_PROJECTION,
    STORAGE_REMOVED_PROJECTION,
];

// -- Local deserialization structs for struct-field events --

#[derive(serde::Deserialize)]
struct NameOnly<T> {
    name: T,
}

#[derive(serde::Deserialize)]
struct KeyOnly {
    key: StorageKey,
}

#[derive(serde::Deserialize)]
struct RefAdded {
    experience_id: ExperienceId,
    record_ref: RecordRef,
}

#[derive(serde::Deserialize)]
struct DescriptionUpdated {
    experience_id: ExperienceId,
    description: Content,
}

// -- System projections --

const TENANT_PROJECTION: Projection = Projection {
    name: "tenant",
    events: &["tenant-created"],
    apply: apply_tenant,
    reset: reset_tenant,
};

fn apply_tenant(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let tenant: Tenant = serde_json::from_value(data.clone())?;
    conn.create_tenant(tenant.tenant_id.to_string(), &tenant.name)?;
    Ok(())
}

fn reset_tenant(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_tenants()?;
    Ok(())
}

const ACTOR_PROJECTION: Projection = Projection {
    name: "actor",
    events: &["actor-created"],
    apply: apply_actor,
    reset: reset_actor,
};

fn apply_actor(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let actor: Actor = serde_json::from_value(data.clone())?;
    conn.create_actor(
        actor.actor_id.to_string(),
        actor.tenant_id.to_string(),
        &actor.name,
    )?;
    Ok(())
}

fn reset_actor(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_actors()?;
    Ok(())
}

const BRAIN_PROJECTION: Projection = Projection {
    name: "brain",
    events: &["brain-created"],
    apply: apply_brain,
    reset: reset_brain,
};

fn apply_brain(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let brain: Brain = serde_json::from_value(data.clone())?;
    conn.create_brain(
        brain.brain_id.to_string(),
        brain.tenant_id.to_string(),
        &brain.name,
        brain.path.display().to_string(),
    )?;
    Ok(())
}

fn reset_brain(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_brains()?;
    Ok(())
}

const TICKET_ISSUED_PROJECTION: Projection = Projection {
    name: "ticket-issued",
    events: &["ticket-issued"],
    apply: apply_ticket_issued,
    reset: reset_tickets,
};

fn apply_ticket_issued(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let ticket: Ticket = serde_json::from_value(data.clone())?;
    conn.create_ticket(
        ticket.ticket_id.to_string(),
        ticket.token.to_string(),
        ticket.created_by.to_string(),
    )?;
    Ok(())
}

fn reset_tickets(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_tickets()?;
    Ok(())
}

// -- Brain projections: name/description/prompt entities --

const PERSONA_SET_PROJECTION: Projection = Projection {
    name: "persona-set",
    events: &["persona-set"],
    apply: apply_persona_set,
    reset: reset_personas,
};

fn apply_persona_set(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let persona: Persona = serde_json::from_value(data.clone())?;
    conn.set_persona(
        &persona.name,
        persona.description.as_str(),
        persona.prompt.as_str(),
    )?;
    Ok(())
}

fn reset_personas(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_personas()?;
    Ok(())
}

const PERSONA_REMOVED_PROJECTION: Projection = Projection {
    name: "persona-removed",
    events: &["persona-removed"],
    apply: apply_persona_removed,
    reset: reset_personas_noop,
};

fn apply_persona_removed(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<PersonaName> = serde_json::from_value(data.clone())?;
    conn.remove_persona(&removed.name)?;
    Ok(())
}

fn reset_personas_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}

const TEXTURE_SET_PROJECTION: Projection = Projection {
    name: "texture-set",
    events: &["texture-set"],
    apply: apply_texture_set,
    reset: reset_textures,
};

fn apply_texture_set(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let texture: Texture = serde_json::from_value(data.clone())?;
    conn.set_texture(
        &texture.name,
        texture.description.as_str(),
        texture.prompt.as_str(),
    )?;
    Ok(())
}

fn reset_textures(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_textures()?;
    Ok(())
}

const TEXTURE_REMOVED_PROJECTION: Projection = Projection {
    name: "texture-removed",
    events: &["texture-removed"],
    apply: apply_texture_removed,
    reset: reset_textures_noop,
};

fn apply_texture_removed(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<TextureName> = serde_json::from_value(data.clone())?;
    conn.remove_texture(&removed.name)?;
    Ok(())
}

fn reset_textures_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}

const LEVEL_SET_PROJECTION: Projection = Projection {
    name: "level-set",
    events: &["level-set"],
    apply: apply_level_set,
    reset: reset_levels,
};

fn apply_level_set(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let level: Level = serde_json::from_value(data.clone())?;
    conn.set_level(
        &level.name,
        level.description.as_str(),
        level.prompt.as_str(),
    )?;
    Ok(())
}

fn reset_levels(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_levels()?;
    Ok(())
}

const LEVEL_REMOVED_PROJECTION: Projection = Projection {
    name: "level-removed",
    events: &["level-removed"],
    apply: apply_level_removed,
    reset: reset_levels_noop,
};

fn apply_level_removed(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<LevelName> = serde_json::from_value(data.clone())?;
    conn.remove_level(&removed.name)?;
    Ok(())
}

fn reset_levels_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}

const SENSATION_SET_PROJECTION: Projection = Projection {
    name: "sensation-set",
    events: &["sensation-set"],
    apply: apply_sensation_set,
    reset: reset_sensations,
};

fn apply_sensation_set(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let sensation: Sensation = serde_json::from_value(data.clone())?;
    conn.set_sensation(
        &sensation.name,
        sensation.description.as_str(),
        sensation.prompt.as_str(),
    )?;
    Ok(())
}

fn reset_sensations(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_sensations()?;
    Ok(())
}

const SENSATION_REMOVED_PROJECTION: Projection = Projection {
    name: "sensation-removed",
    events: &["sensation-removed"],
    apply: apply_sensation_removed,
    reset: reset_sensations_noop,
};

fn apply_sensation_removed(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<SensationName> = serde_json::from_value(data.clone())?;
    conn.remove_sensation(&removed.name)?;
    Ok(())
}

fn reset_sensations_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}

// -- Brain projections: agents --

const AGENT_CREATED_PROJECTION: Projection = Projection {
    name: "agent-created",
    events: &["agent-created"],
    apply: apply_agent_created,
    reset: reset_agents,
};

fn apply_agent_created(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let agent: Agent = serde_json::from_value(data.clone())?;
    conn.create_agent_record(
        agent.id.to_string(),
        &agent.name,
        &agent.persona,
        agent.description.as_str(),
        agent.prompt.as_str(),
    )?;
    Ok(())
}

fn reset_agents(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_agents()?;
    Ok(())
}

const AGENT_UPDATED_PROJECTION: Projection = Projection {
    name: "agent-updated",
    events: &["agent-updated"],
    apply: apply_agent_updated,
    reset: reset_agents_noop,
};

fn apply_agent_updated(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let agent: Agent = serde_json::from_value(data.clone())?;
    conn.update_agent(
        &agent.name,
        &agent.persona,
        agent.description.as_str(),
        agent.prompt.as_str(),
    )?;
    Ok(())
}

fn reset_agents_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}

const AGENT_REMOVED_PROJECTION: Projection = Projection {
    name: "agent-removed",
    events: &["agent-removed"],
    apply: apply_agent_removed,
    reset: reset_agents_removed_noop,
};

fn apply_agent_removed(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<AgentName> = serde_json::from_value(data.clone())?;
    conn.remove_agent(&removed.name)?;
    Ok(())
}

fn reset_agents_removed_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}

// -- Brain projections: cognitions + memories --

const COGNITION_ADDED_PROJECTION: Projection = Projection {
    name: "cognition-added",
    events: &["cognition-added"],
    apply: apply_cognition_added,
    reset: reset_cognitions,
};

fn apply_cognition_added(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let cognition: Cognition = serde_json::from_value(data.clone())?;
    conn.add_cognition(
        cognition.id.to_string(),
        cognition.agent_id.to_string(),
        &cognition.texture,
        cognition.content.as_str(),
        cognition.created_at.to_rfc3339(),
    )?;
    Ok(())
}

fn reset_cognitions(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_cognitions()?;
    Ok(())
}

const MEMORY_ADDED_PROJECTION: Projection = Projection {
    name: "memory-added",
    events: &["memory-added"],
    apply: apply_memory_added,
    reset: reset_memories,
};

fn apply_memory_added(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let memory: Memory = serde_json::from_value(data.clone())?;
    conn.add_memory(
        memory.id.to_string(),
        memory.agent_id.to_string(),
        &memory.level,
        memory.content.as_str(),
        memory.created_at.to_rfc3339(),
    )?;
    Ok(())
}

fn reset_memories(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_memories()?;
    Ok(())
}

// -- Brain projections: storage --

const STORAGE_SET_PROJECTION: Projection = Projection {
    name: "storage-set",
    events: &["storage-set"],
    apply: apply_storage_set,
    reset: reset_storage,
};

fn apply_storage_set(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let entry: StorageEntry = serde_json::from_value(data.clone())?;
    conn.set_storage(&entry.key, entry.description.as_str(), &entry.hash)?;
    Ok(())
}

fn reset_storage(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_storage()?;
    Ok(())
}

const STORAGE_REMOVED_PROJECTION: Projection = Projection {
    name: "storage-removed",
    events: &["storage-removed"],
    apply: apply_storage_removed,
    reset: reset_storage_noop,
};

fn apply_storage_removed(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: KeyOnly = serde_json::from_value(data.clone())?;
    conn.remove_storage(&removed.key)?;
    Ok(())
}

fn reset_storage_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}

// -- Brain projections: experiences --

const EXPERIENCE_CREATED_PROJECTION: Projection = Projection {
    name: "experience-created",
    events: &["experience-created"],
    apply: apply_experience_created,
    reset: reset_experiences,
};

fn apply_experience_created(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let experience: Experience = serde_json::from_value(data.clone())?;
    let id = experience.id.to_string();
    let created_at = experience.created_at.to_rfc3339();

    conn.add_experience(
        &id,
        experience.agent_id.to_string(),
        &experience.sensation,
        experience.description.as_str(),
        &created_at,
    )?;

    for record_ref in &experience.refs {
        conn.add_experience_ref(
            &id,
            record_ref.id().to_string(),
            record_ref.kind().to_string(),
            record_ref.role().map(|l| l.as_str()),
            &created_at,
        )?;
    }

    Ok(())
}

fn reset_experiences(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_experiences()?;
    Ok(())
}

const EXPERIENCE_REF_ADDED_PROJECTION: Projection = Projection {
    name: "experience-ref-added",
    events: &["experience-ref-added"],
    apply: apply_experience_ref_added,
    reset: reset_experience_refs,
};

fn apply_experience_ref_added(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let added: RefAdded = serde_json::from_value(data.clone())?;
    let now = chrono::Utc::now().to_rfc3339();

    conn.add_experience_ref(
        added.experience_id.to_string(),
        added.record_ref.id().to_string(),
        added.record_ref.kind().to_string(),
        added.record_ref.role().map(|l| l.as_str()),
        &now,
    )?;

    Ok(())
}

fn reset_experience_refs(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_experience_refs()?;
    Ok(())
}

const EXPERIENCE_DESCRIPTION_UPDATED_PROJECTION: Projection = Projection {
    name: "experience-description-updated",
    events: &["experience-description-updated"],
    apply: apply_experience_description_updated,
    reset: reset_experience_description_noop,
};

fn apply_experience_description_updated(
    conn: &Database,
    data: &Value,
) -> Result<(), DatabaseError> {
    let updated: DescriptionUpdated = serde_json::from_value(data.clone())?;
    conn.update_experience_description(
        updated.experience_id.to_string(),
        updated.description.as_str(),
    )?;
    Ok(())
}

fn reset_experience_description_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}
