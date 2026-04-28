use crate::*;

pub struct AgentState;

impl AgentState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Agent(agent_event) = event {
            if let Some(agent) = agent_event.maybe_agent() {
                canon.agents.set(&agent);
            };

            if let AgentEvents::AgentRemoved(AgentRemoved::V1(removal)) = agent_event {
                canon.agents.remove_by_name(&removal.name);
            }
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_agent() {
        let agent = Agent::builder()
            .name("test.agent")
            .persona("process")
            .description("A test")
            .prompt("You are a test")
            .build();
        let event = Events::Agent(AgentEvents::AgentCreated(
            AgentCreated::builder_v1().agent(agent).build().into(),
        ));

        let next = AgentState::reduce(BrainCanon::default(), &event);

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

        let event = Events::Agent(AgentEvents::AgentRemoved(AgentRemoved::V1(
            AgentRemovedV1 {
                name: agent.name.clone(),
            },
        )));
        let next = AgentState::reduce(canon, &event);

        assert_eq!(next.agents.len(), 0);
    }
}
