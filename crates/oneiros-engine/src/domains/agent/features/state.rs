use crate::*;

pub struct AgentState;

impl AgentState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        match event {
            Events::Agent(AgentEvents::AgentCreated(agent)) => {
                canon.agents.insert(agent.id.to_string(), agent.clone());
            }
            Events::Agent(AgentEvents::AgentUpdated(agent)) => {
                canon.agents.insert(agent.id.to_string(), agent.clone());
            }
            Events::Agent(AgentEvents::AgentRemoved(removed)) => {
                canon.agents.retain(|_, a| a.name != removed.name);
            }
            _ => {}
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
        assert_eq!(next.agents[&agent.id.to_string()].name, agent.name);
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
        canon.agents.insert(agent.id.to_string(), agent.clone());

        let event = Events::Agent(AgentEvents::AgentRemoved(AgentRemoved {
            name: agent.name.clone(),
        }));
        let next = AgentState::reduce(canon, &event);

        assert!(next.agents.is_empty());
    }
}
