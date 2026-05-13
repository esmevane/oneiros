use crate::*;

pub(crate) struct AgentState;

impl AgentState {
    pub(crate) fn reduce(mut canon: ProjectCanon, event: &Events) -> ProjectCanon {
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

    pub(crate) fn reducer() -> Reducer<ProjectCanon> {
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

        let next = AgentState::reduce(ProjectCanon::default(), &event);

        assert_eq!(next.agents.len(), 1);
    }

    #[test]
    fn removes_agent() {
        let mut canon = ProjectCanon::default();
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
