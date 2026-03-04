use oneiros_db::*;
use oneiros_model::*;
use serde_json::Value;

/// System projections, ordered by dependency (tenant before actor, actor before brain,
/// brain before ticket).
pub const ALL: &[Projection] = &[
    TENANT_PROJECTION,
    ACTOR_PROJECTION,
    BRAIN_PROJECTION,
    TICKET_ISSUED_PROJECTION,
];

const TENANT_PROJECTION: Projection = Projection {
    name: "tenant",
    events: &["tenant-created"],
    apply: apply_tenant,
    reset: |db| db.reset_tenants(),
};

fn apply_tenant(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let tenant: Tenant = serde_json::from_value(data.clone())?;

    db.create_tenant(&tenant.id, &tenant.name)?;

    Ok(())
}

const ACTOR_PROJECTION: Projection = Projection {
    name: "actor",
    events: &["actor-created"],
    apply: apply_actor,
    reset: |db| db.reset_actors(),
};

fn apply_actor(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let actor: Actor = serde_json::from_value(data.clone())?;

    db.create_actor(&actor.id, &actor.tenant_id, &actor.name)?;

    Ok(())
}

const BRAIN_PROJECTION: Projection = Projection {
    name: "brain",
    events: &["brain-created"],
    apply: apply_brain,
    reset: |db| db.reset_brains(),
};

fn apply_brain(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    #[derive(serde::Deserialize)]
    struct BrainEventData {
        id: BrainId,
        tenant_id: TenantId,
        name: BrainName,
        #[serde(default)]
        path: Option<String>,
    }

    let data: BrainEventData = serde_json::from_value(data.clone())?;
    let path = data
        .path
        .unwrap_or_else(|| format!("brains/{}.db", data.name));

    db.create_brain(&data.id, &data.tenant_id, &data.name, &path)?;

    Ok(())
}

const TICKET_ISSUED_PROJECTION: Projection = Projection {
    name: "ticket-issued",
    events: &["ticket-issued"],
    apply: apply_ticket_issued,
    reset: |db| db.reset_tickets(),
};

fn apply_ticket_issued(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let ticket: Ticket = serde_json::from_value(data.clone())?;

    db.create_ticket(
        ticket.id.to_string(),
        ticket.token.to_string(),
        ticket.created_by.to_string(),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn setup_system_db() -> (tempfile::TempDir, Database) {
        let temp = tempfile::TempDir::new().unwrap();
        let db = Database::create(temp.path().join("system.db")).unwrap();
        let tenant_id = TenantId::new();
        db.create_tenant(&tenant_id, &TenantName::new("test"))
            .unwrap();
        (temp, db)
    }

    #[test]
    fn apply_brain_with_absolute_path_preserves_it() {
        let (_temp, db) = setup_system_db();
        let brain_id = BrainId::new();
        let tenant_id: TenantId = db.get_tenant_id().unwrap().unwrap().parse().unwrap();

        let data = json!({
            "id": brain_id.to_string(),
            "tenant_id": tenant_id.to_string(),
            "name": "legacy-brain",
            "status": "active",
            "path": "/absolute/path/to/brain.db"
        });

        apply_brain(&db, &data).unwrap();

        let stored = db
            .get_brain_path(tenant_id.to_string(), brain_id.to_string())
            .unwrap();
        assert_eq!(stored, Some("/absolute/path/to/brain.db".to_string()));
    }

    #[test]
    fn apply_brain_without_path_derives_relative() {
        let (_temp, db) = setup_system_db();
        let brain_id = BrainId::new();
        let tenant_id: TenantId = db.get_tenant_id().unwrap().unwrap().parse().unwrap();

        let data = json!({
            "id": brain_id.to_string(),
            "tenant_id": tenant_id.to_string(),
            "name": "new-brain",
            "status": "active"
        });

        apply_brain(&db, &data).unwrap();

        let stored = db
            .get_brain_path(tenant_id.to_string(), brain_id.to_string())
            .unwrap();
        assert_eq!(stored, Some("brains/new-brain.db".to_string()));
    }
}
