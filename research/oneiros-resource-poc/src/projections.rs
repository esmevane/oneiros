use oneiros_db::{Database, DatabaseError, Projection};
use oneiros_model::*;

/// Minimal projections for the POC.
///
/// These mirror projections from `oneiros-service/src/projections/brain.rs`
/// without pulling in that crate.
pub const AGENT: &[Projection] = &[AGENT_CREATED, AGENT_UPDATED, AGENT_REMOVED];
pub const LEVEL: &[Projection] = &[LEVEL_SET, LEVEL_REMOVED];

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

// ── Level projections ─────────────────────────────────────────────

const LEVEL_SET: Projection = Projection {
    name: "poc:level-set",
    apply: apply_level_set,
    reset: |db| db.reset_levels(),
};

fn apply_level_set(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Level(LevelEvents::LevelSet(level)) = &event.data else {
        return Ok(());
    };

    db.set_level(&level.name, &level.description, &level.prompt)?;

    Ok(())
}

const LEVEL_REMOVED: Projection = Projection {
    name: "poc:level-removed",
    apply: apply_level_removed,
    reset: |_| Ok(()),
};

fn apply_level_removed(db: &Database, event: &KnownEvent) -> Result<(), DatabaseError> {
    let Events::Level(LevelEvents::LevelRemoved(removed)) = &event.data else {
        return Ok(());
    };

    db.remove_level(&removed.name)?;

    Ok(())
}
