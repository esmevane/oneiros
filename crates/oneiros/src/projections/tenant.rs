use oneiros_db::{Database, DatabaseError, Projection};
use serde_json::Value;

pub(super) const PROJECTION: Projection = Projection {
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

fn reset_tenant(conn: &oneiros_db::Database) -> Result<(), DatabaseError> {
    conn.reset_tenants()?;

    Ok(())
}
