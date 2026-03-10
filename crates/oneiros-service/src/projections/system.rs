use oneiros_db::*;
use oneiros_model::*;

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
    apply: apply_tenant,
    reset: |db| db.reset_tenants(),
};

fn apply_tenant(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Tenant(TenantEvents::TenantCreated(tenant)) = &event.data else {
        return Ok(());
    };

    db.create_tenant(&tenant.id, &tenant.name)?;

    Ok(())
}

const ACTOR_PROJECTION: Projection = Projection {
    name: "actor",
    apply: apply_actor,
    reset: |db| db.reset_actors(),
};

fn apply_actor(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Actor(ActorEvents::ActorCreated(actor)) = &event.data else {
        return Ok(());
    };

    db.create_actor(&actor.id, &actor.tenant_id, &actor.name)?;

    Ok(())
}

const BRAIN_PROJECTION: Projection = Projection {
    name: "brain",
    apply: apply_brain,
    reset: |db| db.reset_brains(),
};

fn apply_brain(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Brain(BrainEvents::BrainCreated(brain)) = &event.data else {
        return Ok(());
    };
    let path = brain.path.display().to_string();

    db.create_brain(&brain.id, &brain.tenant_id, &brain.name, &path)?;

    Ok(())
}

const TICKET_ISSUED_PROJECTION: Projection = Projection {
    name: "ticket-issued",
    apply: apply_ticket_issued,
    reset: |db| db.reset_tickets(),
};

fn apply_ticket_issued(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Ticket(TicketEvents::TicketIssued(ticket)) = &event.data else {
        return Ok(());
    };

    db.create_ticket(
        ticket.id.to_string(),
        ticket.token.to_string(),
        ticket.created_by.to_string(),
    )?;

    Ok(())
}
