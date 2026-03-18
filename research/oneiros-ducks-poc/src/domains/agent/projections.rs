use oneiros_db::Projection;
use oneiros_model::*;

pub const PROJECTIONS: &[Projection] = &[CREATED, UPDATED, REMOVED];

const CREATED: Projection = Projection {
    name: "agent-created",
    apply: |db, event| {
        let Events::Agent(AgentEvents::AgentCreated(agent)) = &event.data else {
            return Ok(());
        };
        db.create_agent_record(
            &agent.id,
            &agent.name,
            &agent.persona,
            &agent.description,
            &agent.prompt,
        )
    },
    reset: |db| db.reset_agents(),
};

const UPDATED: Projection = Projection {
    name: "agent-updated",
    apply: |db, event| {
        let Events::Agent(AgentEvents::AgentUpdated(agent)) = &event.data else {
            return Ok(());
        };
        db.update_agent(
            &agent.name,
            &agent.persona,
            &agent.description,
            &agent.prompt,
        )
    },
    reset: |_| Ok(()),
};

const REMOVED: Projection = Projection {
    name: "agent-removed",
    apply: |db, event| {
        let Events::Agent(AgentEvents::AgentRemoved(removed)) = &event.data else {
            return Ok(());
        };
        db.remove_agent(&removed.name)
    },
    reset: |_| Ok(()),
};
