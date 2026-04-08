use std::sync::Arc;

use loro::{ExportMode, LoroDoc};
use lorosurgeon::DocSync;

use crate::*;

/// Reconcile a canon state into a LoroDoc.
///
/// Implemented by BrainCanon and SystemCanon to bridge
/// the reducer output (pure state) into the CRDT document.
pub trait Materialize {
    fn materialize(&self, doc: &LoroDoc) -> Result<(), EventError>;
}

impl Materialize for BrainCanon {
    fn materialize(&self, doc: &LoroDoc) -> Result<(), EventError> {
        self.agents.to_doc(doc)?;
        self.cognitions.to_doc(doc)?;
        self.memories.to_doc(doc)?;
        self.experiences.to_doc(doc)?;
        self.connections.to_doc(doc)?;
        self.storage.to_doc(doc)?;
        self.levels.to_doc(doc)?;
        self.textures.to_doc(doc)?;
        self.sensations.to_doc(doc)?;
        self.natures.to_doc(doc)?;
        self.personas.to_doc(doc)?;
        self.urges.to_doc(doc)?;
        self.pressures.to_doc(doc)?;
        doc.commit();
        Ok(())
    }
}

impl Materialize for SystemCanon {
    fn materialize(&self, doc: &LoroDoc) -> Result<(), EventError> {
        self.actors.to_doc(doc)?;
        self.brains.to_doc(doc)?;
        self.tenants.to_doc(doc)?;
        self.tickets.to_doc(doc)?;
        doc.commit();
        Ok(())
    }
}

/// A CRDT-backed materialization of domain state.
///
/// Canon wraps a Loro document that receives the reducer's output
/// after every event. The reducer produces pure state, Canon
/// reconciles it into the CRDT via lorosurgeon's DocSync.
///
/// This is the distributable unit — snapshots, branching, and
/// eventually multi-host sync all happen at this layer.
#[derive(Clone)]
pub struct Canon {
    doc: Arc<LoroDoc>,
}

impl Default for Canon {
    fn default() -> Self {
        Self::new()
    }
}

impl Canon {
    pub fn new() -> Self {
        Self {
            doc: Arc::new(LoroDoc::new()),
        }
    }

    /// The underlying Loro document.
    pub fn doc(&self) -> &LoroDoc {
        &self.doc
    }

    /// Reconcile canon state into the CRDT document.
    pub fn reconcile<T: Materialize>(&self, state: &T) -> Result<(), EventError> {
        state.materialize(&self.doc)
    }

    /// Export a full snapshot of the document.
    pub fn snapshot(&self) -> Result<Vec<u8>, EventError> {
        self.doc
            .export(ExportMode::Snapshot)
            .map_err(|e| EventError::Import(e.to_string()))
    }

    /// Restore a canon from a snapshot.
    pub fn restore(bytes: &[u8]) -> Result<Self, EventError> {
        let doc = Arc::new(LoroDoc::new());
        doc.import(bytes)?;
        Ok(Self { doc })
    }

    /// Fork this canon into an independent copy.
    ///
    /// The forked doc shares history up to this point but
    /// diverges from here. This is the branching primitive.
    pub fn fork(&self) -> Self {
        Self {
            doc: Arc::new(self.doc.fork()),
        }
    }

    /// Merge another canon's changes into this one.
    ///
    /// Exports all updates from the source and imports them
    /// into this doc. CRDT resolution handles conflicts.
    pub fn merge_from(&self, source: &Canon) -> Result<(), EventError> {
        let updates = source
            .doc
            .export(ExportMode::all_updates())
            .map_err(|e| EventError::Import(e.to_string()))?;
        self.doc.import(&updates)?;
        Ok(())
    }

    /// Clear the document for replay.
    pub fn reset(&self) -> Result<(), EventError> {
        // Create a new empty doc isn't possible with Arc sharing,
        // so we reconcile empty state on next apply.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconcile_brain_canon() {
        let canon = Canon::new();
        let mut state = BrainCanon::default();

        let agent = Agent::builder()
            .name("test.agent")
            .persona("process")
            .description("A test agent")
            .prompt("You are a test")
            .build();
        state.agents.set(&agent);

        let cognition = Cognition::builder()
            .agent_id(AgentId::new())
            .texture("observation")
            .content("Something noticed")
            .build();
        state.cognitions.set(&cognition);

        canon.reconcile(&state).unwrap();

        // Verify round-trip: hydrate back from the doc
        let snapshot = canon.snapshot().unwrap();
        let restored = Canon::restore(&snapshot).unwrap();

        let agents = Agents::from_doc(&restored.doc).unwrap();
        assert_eq!(agents.len(), 1);

        let cognitions = Cognitions::from_doc(&restored.doc).unwrap();
        assert_eq!(cognitions.len(), 1);
    }

    #[test]
    fn reconcile_system_canon() {
        let canon = Canon::new();
        let mut state = SystemCanon::default();

        let tenant = Tenant::builder().name("test-tenant").build();
        state.tenants.set(&tenant);

        canon.reconcile(&state).unwrap();

        let snapshot = canon.snapshot().unwrap();
        let restored = Canon::restore(&snapshot).unwrap();

        let tenants = Tenants::from_doc(&restored.doc).unwrap();
        assert_eq!(tenants.len(), 1);
    }

    #[test]
    fn reconcile_is_incremental() {
        let canon = Canon::new();
        let mut state = BrainCanon::default();

        // First reconcile: one agent
        let agent = Agent::builder()
            .name("first.agent")
            .persona("process")
            .description("First")
            .prompt("You are first")
            .build();
        state.agents.set(&agent);
        canon.reconcile(&state).unwrap();

        // Second reconcile: add a cognition
        let cognition = Cognition::builder()
            .agent_id(AgentId::new())
            .texture("observation")
            .content("Something noticed")
            .build();
        state.cognitions.set(&cognition);
        canon.reconcile(&state).unwrap();

        // Both should be present
        let agents = Agents::from_doc(&canon.doc).unwrap();
        assert_eq!(agents.len(), 1);

        let cognitions = Cognitions::from_doc(&canon.doc).unwrap();
        assert_eq!(cognitions.len(), 1);
    }

    #[test]
    fn snapshot_and_restore() {
        let canon = Canon::new();
        let mut state = BrainCanon::default();

        let agent = Agent::builder()
            .name("test.agent")
            .persona("process")
            .description("A test")
            .prompt("You are a test")
            .build();
        state.agents.set(&agent);
        canon.reconcile(&state).unwrap();

        let snapshot = canon.snapshot().unwrap();
        let restored = Canon::restore(&snapshot).unwrap();

        let agents = Agents::from_doc(&restored.doc).unwrap();
        assert_eq!(agents.len(), 1);
    }

    #[test]
    fn reconcile_empty_clears_state() {
        let canon = Canon::new();
        let mut state = BrainCanon::default();

        // Add data
        let agent = Agent::builder()
            .name("test.agent")
            .persona("process")
            .description("A test")
            .prompt("You are a test")
            .build();
        state.agents.set(&agent);
        canon.reconcile(&state).unwrap();

        let agents = Agents::from_doc(&canon.doc).unwrap();
        assert_eq!(agents.len(), 1);

        // Reconcile with empty state — should clear the doc
        let empty = BrainCanon::default();
        canon.reconcile(&empty).unwrap();

        let agents = Agents::from_doc(&canon.doc).unwrap();
        assert_eq!(agents.len(), 0);
    }

    #[test]
    fn replay_through_reducer_pipeline() {
        // Simulates the replay flow: fold events through reducers,
        // reconcile after each, verify the canon doc has correct data.
        let pipeline = ReducerPipeline::brain();
        let canon = Canon::new();

        let agent = Agent::builder()
            .name("test.agent")
            .persona("process")
            .description("A test")
            .prompt("You are a test")
            .build();
        let cognition = Cognition::builder()
            .agent_id(agent.id)
            .texture("observation")
            .content("First thought")
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

        // Replay: reduce each event, reconcile after each
        for event in &events {
            pipeline.apply(event).unwrap();
            canon.reconcile(&pipeline.state().unwrap()).unwrap();
        }

        // Verify the canon doc
        let agents = Agents::from_doc(&canon.doc).unwrap();
        assert_eq!(agents.len(), 1);

        let cognitions = Cognitions::from_doc(&canon.doc).unwrap();
        assert_eq!(cognitions.len(), 1);

        let levels = Levels::from_doc(&canon.doc).unwrap();
        assert_eq!(levels.len(), 1);

        // Now simulate reset + replay (what Projections::replay does)
        pipeline.reset().unwrap();
        canon.reconcile(&pipeline.state().unwrap()).unwrap();

        // After reset, doc should be empty
        let agents = Agents::from_doc(&canon.doc).unwrap();
        assert_eq!(agents.len(), 0);

        // Replay again
        for event in &events {
            pipeline.apply(event).unwrap();
            canon.reconcile(&pipeline.state().unwrap()).unwrap();
        }

        // Should have the same data as before
        let agents = Agents::from_doc(&canon.doc).unwrap();
        assert_eq!(agents.len(), 1);

        let cognitions = Cognitions::from_doc(&canon.doc).unwrap();
        assert_eq!(cognitions.len(), 1);
    }
}
