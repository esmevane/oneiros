use oneiros_db::{Database, DatabaseError, Projection};
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
/// Ordering: persona before agent (agent has FK to persona).
pub const BRAIN_PROJECTIONS: &[Projection] = &[
    LEVEL_SET_PROJECTION,
    LEVEL_REMOVED_PROJECTION,
    PERSONA_SET_PROJECTION,
    PERSONA_REMOVED_PROJECTION,
    TEXTURE_SET_PROJECTION,
    TEXTURE_REMOVED_PROJECTION,
    AGENT_CREATED_PROJECTION,
    AGENT_UPDATED_PROJECTION,
    AGENT_REMOVED_PROJECTION,
    COGNITION_ADDED_PROJECTION,
    MEMORY_ADDED_PROJECTION,
    STORAGE_SET_PROJECTION,
    STORAGE_REMOVED_PROJECTION,
];

const TENANT_PROJECTION: Projection = Projection {
    name: "tenant",
    events: &["tenant-created"],
    apply: apply_tenant,
    reset: reset_tenant,
};

fn apply_tenant(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    if let Some(tenant_id) = data["tenant_id"].as_str()
        && let Some(name) = data["name"].as_str()
    {
        conn.create_tenant(tenant_id, name)?;
    };

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
    if let Some(tenant_id) = data["tenant_id"].as_str()
        && let Some(name) = data["name"].as_str()
        && let Some(actor_id) = data["actor_id"].as_str()
    {
        conn.create_actor(actor_id, tenant_id, name)?;
    };

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
    if let Some(brain_id) = data["brain_id"].as_str()
        && let Some(tenant_id) = data["tenant_id"].as_str()
        && let Some(name) = data["name"].as_str()
        && let Some(path) = data["path"].as_str()
    {
        conn.create_brain(brain_id, tenant_id, name, path)?;
    };

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
    if let Some(ticket_id) = data["ticket_id"].as_str()
        && let Some(token) = data["token"].as_str()
        && let Some(created_by) = data["created_by"].as_str()
    {
        conn.create_ticket(ticket_id, token, created_by)?;
    };

    Ok(())
}

fn reset_tickets(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_tickets()?;
    Ok(())
}

const PERSONA_SET_PROJECTION: Projection = Projection {
    name: "persona-set",
    events: &["persona-set"],
    apply: apply_persona_set,
    reset: reset_personas,
};

fn apply_persona_set(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    if let Some(name) = data["name"].as_str()
        && let Some(description) = data["description"].as_str()
        && let Some(prompt) = data["prompt"].as_str()
    {
        conn.set_persona(name, description, prompt)?;
    };

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
    if let Some(name) = data["name"].as_str() {
        conn.remove_persona(name)?;
    };

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
    if let Some(name) = data["name"].as_str()
        && let Some(description) = data["description"].as_str()
        && let Some(prompt) = data["prompt"].as_str()
    {
        conn.set_texture(name, description, prompt)?;
    };

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
    if let Some(name) = data["name"].as_str() {
        conn.remove_texture(name)?;
    };

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
    if let Some(name) = data["name"].as_str()
        && let Some(description) = data["description"].as_str()
        && let Some(prompt) = data["prompt"].as_str()
    {
        conn.set_level(name, description, prompt)?;
    };

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
    if let Some(name) = data["name"].as_str() {
        conn.remove_level(name)?;
    };

    Ok(())
}

fn reset_levels_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}

const AGENT_CREATED_PROJECTION: Projection = Projection {
    name: "agent-created",
    events: &["agent-created"],
    apply: apply_agent_created,
    reset: reset_agents,
};

fn apply_agent_created(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    if let Some(id) = data["id"].as_str()
        && let Some(name) = data["name"].as_str()
        && let Some(persona) = data["persona"].as_str()
        && let Some(description) = data["description"].as_str()
        && let Some(prompt) = data["prompt"].as_str()
    {
        conn.create_agent_record(id, name, persona, description, prompt)?;
    };

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
    if let Some(name) = data["name"].as_str()
        && let Some(persona) = data["persona"].as_str()
        && let Some(description) = data["description"].as_str()
        && let Some(prompt) = data["prompt"].as_str()
    {
        conn.update_agent(name, persona, description, prompt)?;
    };

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
    if let Some(name) = data["name"].as_str() {
        conn.remove_agent(name)?;
    };

    Ok(())
}

fn reset_agents_removed_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}

const COGNITION_ADDED_PROJECTION: Projection = Projection {
    name: "cognition-added",
    events: &["cognition-added"],
    apply: apply_cognition_added,
    reset: reset_cognitions,
};

fn apply_cognition_added(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    if let Some(id) = data["id"].as_str()
        && let Some(agent_id) = data["agent_id"].as_str()
        && let Some(texture) = data["texture"].as_str()
        && let Some(content) = data["content"].as_str()
        && let Some(created_at) = data["created_at"].as_str()
    {
        conn.add_cognition(id, agent_id, texture, content, created_at)?;
    };

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
    if let Some(id) = data["id"].as_str()
        && let Some(agent_id) = data["agent_id"].as_str()
        && let Some(level) = data["level"].as_str()
        && let Some(content) = data["content"].as_str()
        && let Some(created_at) = data["created_at"].as_str()
    {
        conn.add_memory(id, agent_id, level, content, created_at)?;
    };

    Ok(())
}

fn reset_memories(conn: &Database) -> Result<(), DatabaseError> {
    conn.reset_memories()?;
    Ok(())
}

const STORAGE_SET_PROJECTION: Projection = Projection {
    name: "storage-set",
    events: &["storage-set"],
    apply: apply_storage_set,
    reset: reset_storage,
};

fn apply_storage_set(conn: &Database, data: &Value) -> Result<(), DatabaseError> {
    if let Some(key) = data["key"].as_str()
        && let Some(description) = data["description"].as_str()
        && let Some(hash) = data["hash"].as_str()
    {
        conn.set_storage(key, description, hash)?;
    };

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
    if let Some(key) = data["key"].as_str() {
        conn.remove_storage(key)?;
    };

    Ok(())
}

fn reset_storage_noop(_conn: &Database) -> Result<(), DatabaseError> {
    Ok(())
}
