use oneiros_db::*;
use oneiros_link::*;
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

    db.create_tenant(&tenant.id, &tenant.name, &tenant.as_link()?)?;

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
    let link = actor.as_link()?;

    db.create_actor(&actor.id, &actor.tenant_id, &actor.name, &link)?;

    Ok(())
}

const BRAIN_PROJECTION: Projection = Projection {
    name: "brain",
    events: &["brain-created"],
    apply: apply_brain,
    reset: |db| db.reset_brains(),
};

fn apply_brain(db: &Database, data: &Value) -> Result<(), DatabaseError> {
    let brain: Brain = serde_json::from_value(data.clone())?;
    let link = brain.as_link()?;
    let path = brain.path.display().to_string();

    db.create_brain(&brain.id, &brain.tenant_id, &brain.name, &path, &link)?;

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
