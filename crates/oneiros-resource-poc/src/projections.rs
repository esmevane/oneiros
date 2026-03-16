use oneiros_db::{Database, DatabaseError, Projection};
use oneiros_model::*;

/// Minimal agent projections for the POC.
///
/// These mirror the agent projections from `oneiros-service/src/projections/brain.rs`
/// without pulling in that crate. They update the agent read model when agent events
/// are emitted.
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

    db.create_agent_record(
        &agent.id,
        &agent.name,
        &agent.persona,
        &agent.description,
        &agent.prompt,
    )?;

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

    db.update_agent(
        &agent.name,
        &agent.persona,
        &agent.description,
        &agent.prompt,
    )?;

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
