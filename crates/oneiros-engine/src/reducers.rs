use std::sync::{Arc, Mutex};

use crate::*;

/// A reducer pipeline: the current state plus the reducers that fold events into it.
#[derive(Clone)]
pub struct ReducerPipeline<T> {
    state: Arc<Mutex<T>>,
    reducers: Vec<Reducer<T>>,
}

impl<T: Clone + Default> ReducerPipeline<T> {
    pub fn new(reducers: Vec<Reducer<T>>) -> Self {
        Self {
            state: Arc::new(Mutex::new(T::default())),
            reducers,
        }
    }

    pub fn apply(&self, event: &Events) {
        let mut guard = self.state.lock().unwrap();
        let state = std::mem::take(&mut *guard);
        let mut next = state;

        for reducer in &self.reducers {
            next = (reducer.reduce)(next, event);
        }

        *guard = next;
    }

    pub fn reduce(&self, events: &Vec<Events>) {
        for event in events {
            self.apply(event);
        }
    }

    pub fn reset(&self) {
        let mut guard = self.state.lock().unwrap();
        *guard = T::default();
    }

    pub fn state(&self) -> T {
        let mut guard = self.state.lock().unwrap();
        std::mem::take(&mut *guard).clone()
    }
}

impl ReducerPipeline<BrainCanon> {
    pub fn brain() -> Self {
        Self::new(vec![
            AgentState::reducer(),
            CognitionState::reducer(),
            MemoryState::reducer(),
            ExperienceState::reducer(),
            ConnectionState::reducer(),
            StorageState::reducer(),
            LevelState::reducer(),
            TextureState::reducer(),
            SensationState::reducer(),
            NatureState::reducer(),
            PersonaState::reducer(),
            UrgeState::reducer(),
            PressureState::reducer(),
        ])
    }
}

impl ReducerPipeline<SystemCanon> {
    pub fn system() -> Self {
        Self::new(vec![
            TenantState::reducer(),
            ActorState::reducer(),
            BrainState::reducer(),
            TicketState::reducer(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brain_reducers_chain_through_full_pipeline() {
        let reducers = ReducerPipeline::brain();

        let agent = Agent::builder()
            .name("test.agent")
            .persona("process")
            .description("A test")
            .prompt("You are a test")
            .build();
        let cognition = Cognition::builder()
            .agent_id(AgentId::new())
            .texture("observation")
            .content("Something noticed")
            .build();
        let level = Level::builder()
            .name("working")
            .description("Short-term")
            .prompt("")
            .build();

        let events = vec![
            Events::Agent(AgentEvents::AgentCreated(agent)),
            Events::Cognition(CognitionEvents::CognitionAdded(cognition)),
            Events::Level(LevelEvents::LevelSet(level)),
        ];

        reducers.reduce(&events);

        let state = reducers.state();

        assert_eq!(state.agents.len(), 1);
        assert_eq!(state.cognitions.len(), 1);
        assert_eq!(state.levels.len(), 1);
    }

    #[test]
    fn system_reducers_chain_through_full_pipeline() {
        let reducers = ReducerPipeline::<SystemCanon>::system();

        let tenant = Tenant::builder().name("test-tenant").build();
        let events = vec![Events::Tenant(TenantEvents::TenantCreated(tenant))];

        reducers.reduce(&events);

        let state = reducers.state();

        assert_eq!(state.tenants.len(), 1);
    }
}
