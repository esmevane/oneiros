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
pub const BRAIN_PROJECTIONS: &[Projection] = &[PERSONA_SET_PROJECTION, PERSONA_REMOVED_PROJECTION];

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
