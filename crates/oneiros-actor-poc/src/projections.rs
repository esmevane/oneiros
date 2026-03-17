//! Projections for the actor POC — identical to the resource POC.
//! Duplicated here to keep the two spikes independent.

use oneiros_db::{Database, DatabaseError, Projection};
use oneiros_model::*;

pub const AGENT: &[Projection] = &[AGENT_CREATED, AGENT_UPDATED, AGENT_REMOVED];

const AGENT_CREATED: Projection = Projection {
    name: "poc:agent-created",
    apply: apply_agent_created,
    reset: |db| db.reset_agents(),
};

fn apply_agent_created(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Agent(AgentEvents::AgentCreated(agent)) = &event.data else {
        return Ok(());
    };
    db.create_agent_record(&agent.id, &agent.name, &agent.persona, &agent.description, &agent.prompt)?;
    Ok(())
}

const AGENT_UPDATED: Projection = Projection {
    name: "poc:agent-updated",
    apply: apply_agent_updated,
    reset: |_| Ok(()),
};

fn apply_agent_updated(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Agent(AgentEvents::AgentUpdated(agent)) = &event.data else {
        return Ok(());
    };
    db.update_agent(&agent.name, &agent.persona, &agent.description, &agent.prompt)?;
    Ok(())
}

const AGENT_REMOVED: Projection = Projection {
    name: "poc:agent-removed",
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
