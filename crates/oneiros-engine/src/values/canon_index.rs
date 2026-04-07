use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::*;

/// A shared index of CRDT canons, keyed by brain name.
///
/// Hydrated at server startup from event logs. Shared across
/// all request handlers via `ServerState`. Each brain gets its
/// own `Canon` that accumulates state as events flow through
/// the reducer pipeline.
#[derive(Clone, Default)]
pub struct CanonIndex {
    brains: Arc<RwLock<HashMap<BrainName, Canon>>>,
    system: Canon,
}

impl CanonIndex {
    pub fn new() -> Self {
        Self {
            brains: Arc::new(RwLock::new(HashMap::new())),
            system: Canon::new(),
        }
    }

    /// The system-level canon.
    pub fn system(&self) -> &Canon {
        &self.system
    }

    /// Get or create a canon for a brain.
    pub fn brain(&self, name: &BrainName) -> Canon {
        let read = self.brains.read().unwrap();
        if let Some(canon) = read.get(name) {
            return canon.clone();
        }
        drop(read);

        let mut write = self.brains.write().unwrap();
        write.entry(name.clone()).or_default().clone()
    }

    /// Hydrate the system canon from its event log.
    pub fn hydrate_system(&self, config: &Config) -> Result<(), EventError> {
        let db = config.system_db()?;
        let events = EventLog::new(&db).load_all()?;
        let pipeline = ReducerPipeline::system();

        for event in &events {
            pipeline.apply(&event.data);
        }

        self.system.reconcile(&pipeline.state())?;

        Ok(())
    }

    /// Hydrate a brain's canon from its event log.
    pub fn hydrate_brain(&self, config: &Config, name: &BrainName) -> Result<(), EventError> {
        let mut brain_config = config.clone();
        brain_config.brain = name.clone();

        let db = brain_config.brain_db()?;
        let events = EventLog::new(&db).load_all()?;
        let pipeline = ReducerPipeline::brain();

        for event in &events {
            pipeline.apply(&event.data);
        }

        let canon = self.brain(name);
        canon.reconcile(&pipeline.state())?;

        Ok(())
    }
}
