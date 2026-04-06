use std::sync::Arc;

use loro::{ExportMode, LoroDoc, LoroList, LoroValue};

use crate::*;

/// A CRDT-backed shadow of the event stream.
///
/// Canon wraps a Loro document that receives every event alongside
/// the SQLite projections. It doesn't replace any read path — it
/// exists as a second backend, proving the polymodal architecture
/// and preparing for distribution (versioning, branching, sync).
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

    /// Append a stored event to the CRDT document.
    pub fn apply(&self, event: &StoredEvent) -> Result<(), EventError> {
        let events = self.events();
        let value = serde_json::to_value(event)?;
        let loro_value = LoroValue::from(value);

        events.push(loro_value)?;
        self.doc.commit();

        Ok(())
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

    /// Clear the document for replay.
    pub fn reset(&self) -> Result<(), EventError> {
        let events = self.events();

        for i in (0..events.len()).rev() {
            events.delete(i, 1)?;
        }

        self.doc.commit();

        Ok(())
    }

    /// The number of events in the document.
    pub fn len(&self) -> usize {
        self.events().len()
    }

    /// An empty canon is one without any events
    pub fn is_empty(&self) -> bool {
        self.events().is_empty()
    }

    fn events(&self) -> LoroList {
        self.doc.get_list("events")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_event(seq: i64, event: impl Into<Events>) -> StoredEvent {
        StoredEvent::builder()
            .id(EventId::new())
            .sequence(seq)
            .data(event.into())
            .source(Source::default())
            .created_at(Timestamp::now())
            .build()
    }

    fn sample_agent_event() -> AgentEvents {
        AgentEvents::AgentCreated(
            Agent::builder()
                .name("test.agent")
                .persona("process")
                .description("A test agent")
                .prompt("You are a test")
                .build(),
        )
    }

    fn sample_cognition_event() -> CognitionEvents {
        CognitionEvents::CognitionAdded(
            Cognition::builder()
                .agent_id(AgentId::new())
                .texture("observation")
                .content("Something noticed")
                .build(),
        )
    }

    #[test]
    fn apply_tracks_events() {
        let canon = Canon::new();

        canon.apply(&test_event(1, sample_agent_event())).unwrap();
        canon
            .apply(&test_event(2, sample_cognition_event()))
            .unwrap();

        assert_eq!(canon.len(), 2);
    }

    #[test]
    fn snapshot_and_restore() {
        let canon = Canon::new();

        canon.apply(&test_event(1, sample_agent_event())).unwrap();
        canon
            .apply(&test_event(2, sample_cognition_event()))
            .unwrap();

        let snapshot = canon.snapshot().unwrap();
        let restored = Canon::restore(&snapshot).unwrap();

        assert_eq!(restored.len(), 2);
    }

    #[test]
    fn reset_clears_events() {
        let canon = Canon::new();

        canon.apply(&test_event(1, sample_agent_event())).unwrap();
        canon
            .apply(&test_event(2, sample_cognition_event()))
            .unwrap();
        assert_eq!(canon.len(), 2);

        canon.reset().unwrap();
        assert_eq!(canon.len(), 0);
    }

    #[test]
    fn reset_then_replay() {
        let canon = Canon::new();
        let events = vec![
            test_event(1, sample_agent_event()),
            test_event(2, sample_cognition_event()),
        ];

        for event in &events {
            canon.apply(event).unwrap();
        }
        assert_eq!(canon.len(), 2);

        // Reset and replay — should produce identical count
        canon.reset().unwrap();
        for event in &events {
            canon.apply(event).unwrap();
        }
        assert_eq!(canon.len(), 2);
    }

    #[test]
    fn clone_shares_state() {
        let canon = Canon::new();

        canon.apply(&test_event(1, sample_agent_event())).unwrap();

        let cloned = canon.clone();
        assert_eq!(cloned.len(), 1);

        // Writes through either handle are visible to both
        canon
            .apply(&test_event(2, sample_cognition_event()))
            .unwrap();
        assert_eq!(canon.len(), 2);
        assert_eq!(cloned.len(), 2);
    }
}
