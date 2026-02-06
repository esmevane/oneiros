use oneiros_db::{DatabaseError, Projection};
use serde_json::Value;

pub(super) const PROJECTION: Projection = Projection {
    name: "actor",
    events: &["actor-created"],
    apply: apply_actor,
    reset: reset_actor,
};

fn apply_actor(conn: &oneiros_db::Database, data: &Value) -> Result<(), DatabaseError> {
    if let Some(tenant_id) = data["tenant_id"].as_str()
        && let Some(name) = data["name"].as_str()
        && let Some(actor_id) = data["actor_id"].as_str()
    {
        conn.create_actor(actor_id, tenant_id, name)?;
    };

    Ok(())
}

fn reset_actor(conn: &oneiros_db::Database) -> Result<(), DatabaseError> {
    conn.reset_actors()?;
    Ok(())
}
