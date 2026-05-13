use std::sync::{Arc, Mutex};

use crate::*;

/// A reducer pipeline: the current state plus the reducers that fold events into it.
#[derive(Clone)]
pub(crate) struct ReducerPipeline<T> {
    state: Arc<Mutex<T>>,
    reducers: Vec<Reducer<T>>,
}

impl<T: Clone + Default> ReducerPipeline<T> {
    pub(crate) fn new(reducers: Vec<Reducer<T>>) -> Self {
        Self {
            state: Arc::new(Mutex::new(T::default())),
            reducers,
        }
    }

    pub(crate) fn apply(&self, event: &Events) -> Result<(), EventError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        let state = std::mem::take(&mut *guard);
        let mut next = state;

        for reducer in &self.reducers {
            next = (reducer.reduce)(next, event);
        }

        *guard = next;
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|e| EventError::Lock(e.to_string()))?;
        *guard = T::default();
        Ok(())
    }

    pub(crate) fn state(&self) -> Result<T, EventError> {
        let guard = self
            .state
            .lock()
            .map_err(|e| EventError::Lock(e.to_string()))?;
        Ok(guard.clone())
    }
}

impl ReducerPipeline<ProjectCanon> {
    pub(crate) fn project_with_state(initial: ProjectCanon) -> Result<Self, EventError> {
        let pipeline = Self::project();
        {
            let mut guard = pipeline
                .state
                .lock()
                .map_err(|e| EventError::Lock(e.to_string()))?;
            *guard = initial;
        }
        Ok(pipeline)
    }

    pub(crate) fn project() -> Self {
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

impl ReducerPipeline<HostCanon> {
    pub(crate) fn host() -> Self {
        Self::new(vec![
            TenantState::reducer(),
            ActorState::reducer(),
            ProjectState::reducer(),
            TicketState::reducer(),
            PeerState::reducer(),
            FollowState::reducer(),
        ])
    }
}
