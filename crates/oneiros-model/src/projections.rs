use oneiros_db::{Database, DatabaseError, Projection};
use serde_json::Value;

/// System projections, ordered by dependency (tenant before actor, actor before brain).
pub const SYSTEM_PROJECTIONS: &[Projection] =
    &[TENANT_PROJECTION, ACTOR_PROJECTION, BRAIN_PROJECTION];

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
