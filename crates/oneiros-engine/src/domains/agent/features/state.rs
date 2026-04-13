use crate::*;

pub(crate) struct AgentState;

impl AgentState {
    pub(crate) fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Agent(agent_event) = event {
            match agent_event {
                AgentEvents::AgentCreated(agent) => {
                    canon.agents.set(agent);
                }
                AgentEvents::AgentUpdated(agent) => {
                    canon.agents.set(agent);
                }
                AgentEvents::AgentRemoved(removed) => {
                    canon.agents.remove_by_name(&removed.name);
                }
            };
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_agent() {
        let canon = BrainCanon::default();
        let agent = Agent::builder()
            .name("test.agent")
            .persona("process")
            .description("A test")
            .prompt("You are a test")
            .build();
        let event = Events::Agent(AgentEvents::AgentCreated(agent.clone()));

        let next = AgentState::reduce(canon, &event);

        assert_eq!(next.agents.len(), 1);
    }

    #[test]
    fn removes_agent() {
        let mut canon = BrainCanon::default();
        let agent = Agent::builder()
            .name("test.agent")
            .persona("process")
            .description("A test")
            .prompt("You are a test")
            .build();
        canon.agents.set(&agent);

        let event = Events::Agent(AgentEvents::AgentRemoved(AgentRemoved {
            name: agent.name.clone(),
        }));
        let next = AgentState::reduce(canon, &event);

        assert_eq!(next.agents.len(), 0);
    }
}
