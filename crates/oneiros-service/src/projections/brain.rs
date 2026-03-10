use oneiros_db::*;
use oneiros_model::*;

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
    EXPERIENCE_DESCRIPTION_UPDATED_PROJECTION,
    EXPERIENCE_SENSATION_UPDATED_PROJECTION,
    BLOB_STORED_PROJECTION,
    STORAGE_SET_PROJECTION,
    STORAGE_REMOVED_PROJECTION,
];

const PERSONA_SET_PROJECTION: Projection = Projection {
    name: "persona-set",
    apply: apply_persona_set,
    reset: |db| db.reset_personas(),
};

fn apply_persona_set(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Persona(PersonaEvents::PersonaSet(persona)) = &event.data else {
        return Ok(());
    };

    db.set_persona(&persona.name, &persona.description, &persona.prompt)?;

    Ok(())
}

const PERSONA_REMOVED_PROJECTION: Projection = Projection {
    name: "persona-removed",
    apply: apply_persona_removed,
    reset: |_| Ok(()),
};

fn apply_persona_removed(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Persona(PersonaEvents::PersonaRemoved(removed)) = &event.data else {
        return Ok(());
    };

    db.remove_persona(&removed.name)?;

    Ok(())
}

const TEXTURE_SET_PROJECTION: Projection = Projection {
    name: "texture-set",
    apply: apply_texture_set,
    reset: |db| db.reset_textures(),
};

fn apply_texture_set(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Texture(TextureEvents::TextureSet(texture)) = &event.data else {
        return Ok(());
    };

    db.set_texture(&texture.name, &texture.description, &texture.prompt)?;

    Ok(())
}

const TEXTURE_REMOVED_PROJECTION: Projection = Projection {
    name: "texture-removed",
    apply: apply_texture_removed,
    reset: |_| Ok(()),
};

fn apply_texture_removed(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Texture(TextureEvents::TextureRemoved(removed)) = &event.data else {
        return Ok(());
    };

    db.remove_texture(&removed.name)?;

    Ok(())
}

const LEVEL_SET_PROJECTION: Projection = Projection {
    name: "level-set",
    apply: apply_level_set,
    reset: |db| db.reset_levels(),
};

fn apply_level_set(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Level(LevelEvents::LevelSet(level)) = &event.data else {
        return Ok(());
    };

    db.set_level(&level.name, &level.description, &level.prompt)?;

    Ok(())
}

const LEVEL_REMOVED_PROJECTION: Projection = Projection {
    name: "level-removed",
    apply: apply_level_removed,
    reset: |_| Ok(()),
};

fn apply_level_removed(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Level(LevelEvents::LevelRemoved(removed)) = &event.data else {
        return Ok(());
    };

    db.remove_level(&removed.name)?;

    Ok(())
}

const SENSATION_SET_PROJECTION: Projection = Projection {
    name: "sensation-set",
    apply: apply_sensation_set,
    reset: |db| db.reset_sensations(),
};

fn apply_sensation_set(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Sensation(SensationEvents::SensationSet(sensation)) = &event.data else {
        return Ok(());
    };

    db.set_sensation(&sensation.name, &sensation.description, &sensation.prompt)?;

    Ok(())
}

const SENSATION_REMOVED_PROJECTION: Projection = Projection {
    name: "sensation-removed",
    apply: apply_sensation_removed,
    reset: |_| Ok(()),
};

fn apply_sensation_removed(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Sensation(SensationEvents::SensationRemoved(removed)) = &event.data else {
        return Ok(());
    };

    db.remove_sensation(&removed.name)?;

    Ok(())
}

const NATURE_SET_PROJECTION: Projection = Projection {
    name: "nature-set",
    apply: apply_nature_set,
    reset: |db| db.reset_natures(),
};

fn apply_nature_set(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Nature(NatureEvents::NatureSet(nature)) = &event.data else {
        return Ok(());
    };

    db.set_nature(&nature.name, &nature.description, &nature.prompt)?;

    Ok(())
}

const NATURE_REMOVED_PROJECTION: Projection = Projection {
    name: "nature-removed",
    apply: apply_nature_removed,
    reset: |_| Ok(()),
};

fn apply_nature_removed(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Nature(NatureEvents::NatureRemoved(removed)) = &event.data else {
        return Ok(());
    };

    db.remove_nature(&removed.name)?;

    Ok(())
}

const CONNECTION_CREATED_PROJECTION: Projection = Projection {
    name: "connection-created",
    apply: apply_connection_created,
    reset: |db| db.reset_connections(),
};

fn apply_connection_created(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Connection(ConnectionEvents::ConnectionCreated(connection)) = &event.data else {
        return Ok(());
    };
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
    apply: apply_connection_removed,
    reset: |_| Ok(()),
};

fn apply_connection_removed(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Connection(ConnectionEvents::ConnectionRemoved(removed)) = &event.data else {
        return Ok(());
    };

    db.remove_connection(removed.id.to_string())?;

    Ok(())
}

const AGENT_CREATED_PROJECTION: Projection = Projection {
    name: "agent-created",
    apply: apply_agent_created,
    reset: |db| db.reset_agents(),
};

fn apply_agent_created(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Agent(AgentEvents::AgentCreated(agent)) = &event.data else {
        return Ok(());
    };

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
    apply: apply_agent_updated,
    reset: |_| Ok(()),
};

fn apply_agent_updated(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Agent(AgentEvents::AgentUpdated(agent)) = &event.data else {
        return Ok(());
    };

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
    apply: apply_agent_removed,
    reset: |_| Ok(()),
};

fn apply_agent_removed(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Agent(AgentEvents::AgentRemoved(removed)) = &event.data else {
        return Ok(());
    };

    db.remove_agent(&removed.name)?;

    Ok(())
}

const COGNITION_ADDED_PROJECTION: Projection = Projection {
    name: "cognition-added",
    apply: apply_cognition_added,
    reset: |db| db.reset_cognitions(),
};

fn apply_cognition_added(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Cognition(CognitionEvents::CognitionAdded(cognition)) = &event.data else {
        return Ok(());
    };
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
    apply: apply_memory_added,
    reset: |db| db.reset_memories(),
};

fn apply_memory_added(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Memory(MemoryEvents::MemoryAdded(memory)) = &event.data else {
        return Ok(());
    };
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

const BLOB_STORED_PROJECTION: Projection = Projection {
    name: "blob-stored",
    apply: apply_blob_stored,
    reset: |db| db.reset_blobs(),
};

fn apply_blob_stored(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Storage(StorageEvents::BlobStored(content)) = &event.data else {
        return Ok(());
    };

    db.put_blob(content)?;
    db.delete_blob_stored_event(&content.hash)?;

    Ok(())
}

const STORAGE_SET_PROJECTION: Projection = Projection {
    name: "storage-set",
    apply: apply_storage_set,
    reset: |db| db.reset_storage(),
};

fn apply_storage_set(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Storage(StorageEvents::StorageSet(entry)) = &event.data else {
        return Ok(());
    };

    match db.set_storage(&entry.key, &entry.description, &entry.hash) {
        Ok(()) => Ok(()),
        Err(err) if err.is_foreign_key_violation() => Ok(()),
        Err(err) => Err(err),
    }
}

const STORAGE_REMOVED_PROJECTION: Projection = Projection {
    name: "storage-removed",
    apply: apply_storage_removed,
    reset: |_| Ok(()),
};

fn apply_storage_removed(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Storage(StorageEvents::StorageRemoved(removed)) = &event.data else {
        return Ok(());
    };

    db.remove_storage(&removed.key)?;

    Ok(())
}

const EXPERIENCE_CREATED_PROJECTION: Projection = Projection {
    name: "experience-created",
    apply: apply_experience_created,
    reset: |db| db.reset_experiences(),
};

fn apply_experience_created(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Experience(ExperienceEvents::ExperienceCreated(experience)) = &event.data else {
        return Ok(());
    };
    let created_at = experience.created_at.as_string();

    db.add_experience(
        &experience.id,
        &experience.agent_id,
        &experience.sensation,
        &experience.description,
        &created_at,
    )?;

    Ok(())
}

const EXPERIENCE_DESCRIPTION_UPDATED_PROJECTION: Projection = Projection {
    name: "experience-description-updated",
    apply: apply_experience_description_updated,
    reset: |_| Ok(()),
};

fn apply_experience_description_updated(
    db: &Database,
    event: &KnownEvent,
) -> Result<(), DatabaseError> {
    let Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(updated)) = &event.data
    else {
        return Ok(());
    };

    db.update_experience_description(
        updated.experience_id.to_string(),
        updated.description.as_str(),
    )?;

    Ok(())
}

const EXPERIENCE_SENSATION_UPDATED_PROJECTION: Projection = Projection {
    name: "experience-sensation-updated",
    apply: apply_experience_sensation_updated,
    reset: |_| Ok(()),
};

fn apply_experience_sensation_updated(
    db: &Database,
    event: &KnownEvent,
) -> Result<(), DatabaseError> {
    let Events::Experience(ExperienceEvents::ExperienceSensationUpdated(updated)) = &event.data
    else {
        return Ok(());
    };

    db.update_experience_sensation(
        updated.experience_id.to_string(),
        updated.sensation.as_str(),
    )?;

    Ok(())
}
