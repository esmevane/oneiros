use oneiros_db::*;
use oneiros_model::*;
use serde_json::Value;

#[derive(serde::Deserialize)]
struct NameOnly<T> {
    name: T,
}

#[derive(serde::Deserialize)]
struct KeyOnly {
    key: StorageKey,
}

#[derive(serde::Deserialize)]
struct IdOnly<T> {
    id: T,
}

#[derive(serde::Deserialize)]
struct RefAdded {
    experience_id: ExperienceId,
    experience_ref: ExperienceRef,
    created_at: Timestamp,
}

#[derive(serde::Deserialize)]
struct DescriptionUpdated {
    experience_id: ExperienceId,
    description: Description,
}

/// Brain-level projections for data that lives within each brain's database.
///
/// Ordering: persona before agent (agent has FK to persona),
/// sensation before experience (experience has FK to sensation).
pub const ALL: &[Projection] = &[
    LEVEL_SET_PROJECTION,
    LEVEL_REMOVED_PROJECTION,
    PERSONA_SET_PROJECTION,
    PERSONA_REMOVED_PROJECTION,
    TEXTURE_SET_PROJECTION,
    TEXTURE_REMOVED_PROJECTION,
    SENSATION_SET_PROJECTION,
    SENSATION_REMOVED_PROJECTION,
    NATURE_SET_PROJECTION,
    NATURE_REMOVED_PROJECTION,
    CONNECTION_CREATED_PROJECTION,
    CONNECTION_REMOVED_PROJECTION,
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

const PERSONA_SET_PROJECTION: Projection = Projection {
    name: "persona-set",
    events: &["persona-set"],
    apply: apply_persona_set,
    reset: |db| db.reset_personas(),
};

fn apply_persona_set(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let persona: Persona = serde_json::from_value(data.clone())?;

    db.set_persona(&persona.name, &persona.description, &persona.prompt)?;

    Ok(())
}

const PERSONA_REMOVED_PROJECTION: Projection = Projection {
    name: "persona-removed",
    events: &["persona-removed"],
    apply: apply_persona_removed,
    reset: |_| Ok(()),
};

fn apply_persona_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<PersonaName> = serde_json::from_value(data.clone())?;

    db.remove_persona(&removed.name)?;

    Ok(())
}

const TEXTURE_SET_PROJECTION: Projection = Projection {
    name: "texture-set",
    events: &["texture-set"],
    apply: apply_texture_set,
    reset: |db| db.reset_textures(),
};

fn apply_texture_set(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let texture: Texture = serde_json::from_value(data.clone())?;

    db.set_texture(&texture.name, &texture.description, &texture.prompt)?;

    Ok(())
}

const TEXTURE_REMOVED_PROJECTION: Projection = Projection {
    name: "texture-removed",
    events: &["texture-removed"],
    apply: apply_texture_removed,
    reset: |_| Ok(()),
};

fn apply_texture_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<TextureName> = serde_json::from_value(data.clone())?;

    db.remove_texture(&removed.name)?;

    Ok(())
}

const LEVEL_SET_PROJECTION: Projection = Projection {
    name: "level-set",
    events: &["level-set"],
    apply: apply_level_set,
    reset: |db| db.reset_levels(),
};

fn apply_level_set(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let level: Level = serde_json::from_value(data.clone())?;

    db.set_level(&level.name, &level.description, &level.prompt)?;

    Ok(())
}

const LEVEL_REMOVED_PROJECTION: Projection = Projection {
    name: "level-removed",
    events: &["level-removed"],
    apply: apply_level_removed,
    reset: |_| Ok(()),
};

fn apply_level_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<LevelName> = serde_json::from_value(data.clone())?;

    db.remove_level(&removed.name)?;

    Ok(())
}

const SENSATION_SET_PROJECTION: Projection = Projection {
    name: "sensation-set",
    events: &["sensation-set"],
    apply: apply_sensation_set,
    reset: |db| db.reset_sensations(),
};

fn apply_sensation_set(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let sensation: Sensation = serde_json::from_value(data.clone())?;

    db.set_sensation(&sensation.name, &sensation.description, &sensation.prompt)?;

    Ok(())
}

const SENSATION_REMOVED_PROJECTION: Projection = Projection {
    name: "sensation-removed",
    events: &["sensation-removed"],
    apply: apply_sensation_removed,
    reset: |_| Ok(()),
};

fn apply_sensation_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<SensationName> = serde_json::from_value(data.clone())?;

    db.remove_sensation(&removed.name)?;

    Ok(())
}

const NATURE_SET_PROJECTION: Projection = Projection {
    name: "nature-set",
    events: &["nature-set"],
    apply: apply_nature_set,
    reset: |db| db.reset_natures(),
};

fn apply_nature_set(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let nature: Nature = serde_json::from_value(data.clone())?;

    db.set_nature(&nature.name, &nature.description, &nature.prompt)?;

    Ok(())
}

const NATURE_REMOVED_PROJECTION: Projection = Projection {
    name: "nature-removed",
    events: &["nature-removed"],
    apply: apply_nature_removed,
    reset: |_| Ok(()),
};

fn apply_nature_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<NatureName> = serde_json::from_value(data.clone())?;

    db.remove_nature(&removed.name)?;

    Ok(())
}

const CONNECTION_CREATED_PROJECTION: Projection = Projection {
    name: "connection-created",
    events: &["connection-created"],
    apply: apply_connection_created,
    reset: |db| db.reset_connections(),
};

fn apply_connection_created(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let connection: Connection = serde_json::from_value(data.clone())?;
    let created_at = connection.created_at.as_string();

    db.create_connection(
        &connection.id,
        &connection.nature,
        &connection.from_ref,
        &connection.to_ref,
        &created_at,
    )?;
    Ok(())
}

const CONNECTION_REMOVED_PROJECTION: Projection = Projection {
    name: "connection-removed",
    events: &["connection-removed"],
    apply: apply_connection_removed,
    reset: |_| Ok(()),
};

fn apply_connection_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: IdOnly<ConnectionId> = serde_json::from_value(data.clone())?;

    db.remove_connection(removed.id.to_string())?;

    Ok(())
}

const AGENT_CREATED_PROJECTION: Projection = Projection {
    name: "agent-created",
    events: &["agent-created"],
    apply: apply_agent_created,
    reset: |db| db.reset_agents(),
};

fn apply_agent_created(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let agent: Agent = serde_json::from_value(data.clone())?;

    db.create_agent_record(
        &agent.id,
        &agent.name,
        &agent.persona,
        &agent.description,
        &agent.prompt,
    )?;

    Ok(())
}

const AGENT_UPDATED_PROJECTION: Projection = Projection {
    name: "agent-updated",
    events: &["agent-updated"],
    apply: apply_agent_updated,
    reset: |_| Ok(()),
};

fn apply_agent_updated(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let agent: Agent = serde_json::from_value(data.clone())?;

    db.update_agent(
        &agent.name,
        &agent.persona,
        &agent.description,
        &agent.prompt,
    )?;

    Ok(())
}

const AGENT_REMOVED_PROJECTION: Projection = Projection {
    name: "agent-removed",
    events: &["agent-removed"],
    apply: apply_agent_removed,
    reset: |_| Ok(()),
};

fn apply_agent_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: NameOnly<AgentName> = serde_json::from_value(data.clone())?;

    db.remove_agent(&removed.name)?;

    Ok(())
}

const COGNITION_ADDED_PROJECTION: Projection = Projection {
    name: "cognition-added",
    events: &["cognition-added"],
    apply: apply_cognition_added,
    reset: |db| db.reset_cognitions(),
};

fn apply_cognition_added(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let cognition: Cognition = serde_json::from_value(data.clone())?;
    let created_at = cognition.created_at.as_string();

    db.add_cognition(
        &cognition.id,
        &cognition.agent_id,
        &cognition.texture,
        &cognition.content,
        &created_at,
    )?;

    Ok(())
}

const MEMORY_ADDED_PROJECTION: Projection = Projection {
    name: "memory-added",
    events: &["memory-added"],
    apply: apply_memory_added,
    reset: |db| db.reset_memories(),
};

fn apply_memory_added(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let memory: Memory = serde_json::from_value(data.clone())?;
    let created_at = memory.created_at.as_string();

    db.add_memory(
        &memory.id,
        &memory.agent_id,
        &memory.level,
        &memory.content,
        &created_at,
    )?;

    Ok(())
}

const STORAGE_SET_PROJECTION: Projection = Projection {
    name: "storage-set",
    events: &["storage-set"],
    apply: apply_storage_set,
    reset: |db| db.reset_storage(),
};

fn apply_storage_set(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let entry: StorageEntry = serde_json::from_value(data.clone())?;

    db.set_storage(&entry.key, &entry.description, &entry.hash)?;

    Ok(())
}

const STORAGE_REMOVED_PROJECTION: Projection = Projection {
    name: "storage-removed",
    events: &["storage-removed"],
    apply: apply_storage_removed,
    reset: |_| Ok(()),
};

fn apply_storage_removed(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let removed: KeyOnly = serde_json::from_value(data.clone())?;

    db.remove_storage(&removed.key)?;

    Ok(())
}

const EXPERIENCE_CREATED_PROJECTION: Projection = Projection {
    name: "experience-created",
    events: &["experience-created"],
    apply: apply_experience_created,
    reset: |db| db.reset_experiences(),
};

fn apply_experience_created(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    let experience: Experience = serde_json::from_value(data.clone())?;
    let created_at = experience.created_at.as_string();

    conn.add_experience(
        &experience.id,
        &experience.agent_id,
        &experience.sensation,
        &experience.description,
        &created_at,
    )?;

    for experience_ref in &experience.refs {
        conn.add_experience_ref(experience.id.to_string(), experience_ref, &created_at)?;
    }

    Ok(())
}

const EXPERIENCE_REF_ADDED_PROJECTION: Projection = Projection {
    name: "experience-ref-added",
    events: &["experience-ref-added"],
    apply: apply_experience_ref_added,
    reset: |db| db.reset_experience_refs(),
};

fn apply_experience_ref_added(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let added: RefAdded = serde_json::from_value(data.clone())?;
    let created_at = added.created_at.as_string();

    db.add_experience_ref(
        added.experience_id.to_string(),
        &added.experience_ref,
        &created_at,
    )?;

    Ok(())
}

const EXPERIENCE_DESCRIPTION_UPDATED_PROJECTION: Projection = Projection {
    name: "experience-description-updated",
    events: &["experience-description-updated"],
    apply: apply_experience_description_updated,
    reset: |_| Ok(()),
};

fn apply_experience_description_updated(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let updated: DescriptionUpdated = serde_json::from_value(data.clone())?;

    db.update_experience_description(
        updated.experience_id.to_string(),
        updated.description.as_str(),
    )?;

    Ok(())
}
