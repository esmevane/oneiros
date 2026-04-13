use std::sync::{Arc, Mutex};

use crate::*;

/// A bookmark's record of witnessed events.
///
/// Wraps a ledger root hash — the HAMT pointer that identifies
/// which events this bookmark has seen. Shared via Arc<Mutex> so
/// multiple ProjectContexts (per-request) can chronicle events
/// into the same bookmark.
///
/// Forking snapshots the root (free structural sharing).
/// Diffing two chronicles produces the minimal set of event
/// changes between them.
#[derive(Clone, Debug)]
pub(crate) struct Chronicle {
    root: Arc<Mutex<Option<ContentHash>>>,
}

impl Default for Chronicle {
    fn default() -> Self {
        Self::new()
    }
}

impl Chronicle {
    pub(crate) fn new() -> Self {
        Self {
            root: Arc::new(Mutex::new(None)),
        }
    }

    /// The current root hash, if any events have been recorded.
    pub(crate) fn root(&self) -> Result<Option<ContentHash>, EventError> {
        let guard = self
            .root
            .lock()
            .map_err(|e| EventError::Lock(e.to_string()))?;
        Ok(guard.clone())
    }

    /// Record an event in this chronicle.
    pub(crate) fn record(
        &self,
        event: &StoredEvent,
        resolve: &impl Fn(&ContentHash) -> Option<LedgerNode>,
        store: &impl Fn(&LedgerNode) -> ContentHash,
    ) -> Result<(), EventError> {
        let event_hash = ContentHash::compute(&serde_json::to_vec(&event.data).unwrap_or_default());

        let mut guard = self
            .root
            .lock()
            .map_err(|e| EventError::Lock(e.to_string()))?;

        *guard = Some(Ledger::record(
            guard.as_ref(),
            &event.id,
            event_hash,
            resolve,
            store,
        ));

        Ok(())
    }

    /// Diff this chronicle against another.
    /// Returns changes needed to go from `self` to `other`.
    pub(crate) fn diff(
        &self,
        other: &Chronicle,
        resolve: &impl Fn(&ContentHash) -> Option<LedgerNode>,
    ) -> Result<Vec<LedgerChange>, EventError> {
        let self_root = self
            .root
            .lock()
            .map_err(|e| EventError::Lock(e.to_string()))?
            .clone();
        let other_root = other
            .root
            .lock()
            .map_err(|e| EventError::Lock(e.to_string()))?
            .clone();
        Ok(Ledger::diff(
            self_root.as_ref(),
            other_root.as_ref(),
            resolve,
        ))
    }

    /// Fork this chronicle — snapshots the current root hash.
    /// The forked chronicle is independent from this point forward,
    /// but shares all HAMT structure up to the fork point.
    pub(crate) fn fork(&self) -> Result<Self, EventError> {
        let root = self
            .root
            .lock()
            .map_err(|e| EventError::Lock(e.to_string()))?
            .clone();
        Ok(Self {
            root: Arc::new(Mutex::new(root)),
        })
    }

    /// Merge another chronicle's events into this one.
    /// Records all event IDs from the other chronicle that aren't already present.
    pub(crate) fn merge(
        &self,
        other: &Chronicle,
        resolve: &impl Fn(&ContentHash) -> Option<LedgerNode>,
        store: &impl Fn(&LedgerNode) -> ContentHash,
    ) -> Result<(), EventError> {
        let other_root = other
            .root
            .lock()
            .map_err(|e| EventError::Lock(e.to_string()))?
            .clone();

        if let Some(other_hash) = other_root {
            let other_entries = Ledger::collect_all(&other_hash, resolve);
            let mut guard = self
                .root
                .lock()
                .map_err(|e| EventError::Lock(e.to_string()))?;

            for (key, value) in other_entries {
                if let Ok(event_id) = key.parse() {
                    *guard = Some(Ledger::record(
                        guard.as_ref(),
                        &event_id,
                        value,
                        resolve,
                        store,
                    ));
                }
            }
        }

        Ok(())
    }

    /// Whether this chronicle has any events.
    pub(crate) fn is_empty(&self) -> Result<bool, EventError> {
        let guard = self
            .root
            .lock()
            .map_err(|e| EventError::Lock(e.to_string()))?;
        Ok(guard.is_none())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn memory_store() -> (
        impl Fn(&LedgerNode) -> ContentHash,
        impl Fn(&ContentHash) -> Option<LedgerNode>,
    ) {
        let store: Arc<Mutex<HashMap<String, LedgerNode>>> = Arc::new(Mutex::new(HashMap::new()));

        let store_write = store.clone();
        let put = move |node: &LedgerNode| {
            let bytes = serde_json::to_vec(node).unwrap();
            let hash = ContentHash::compute(&bytes);
            store_write
                .lock()
                .unwrap()
                .insert(hash.to_string(), node.clone());
            hash
        };

        let get = move |hash: &ContentHash| store.lock().unwrap().get(&hash.to_string()).cloned();

        (put, get)
    }

    fn test_event(seq: i64) -> StoredEvent {
        StoredEvent::builder()
            .id(EventId::new())
            .sequence(seq)
            .data(Events::Cognition(CognitionEvents::CognitionAdded(
                Cognition::builder()
                    .agent_id(AgentId::new())
                    .texture("observation")
                    .content(format!("thought {seq}"))
                    .build(),
            )))
            .source(Source::default())
            .created_at(Timestamp::now())
            .build()
    }

    #[test]
    fn chronicle_records_and_diffs() {
        let (store, resolve) = memory_store();

        let main = Chronicle::new();
        let event1 = test_event(1);
        let event2 = test_event(2);

        main.record(&event1, &resolve, &store).unwrap();
        main.record(&event2, &resolve, &store).unwrap();

        let experiment = main.fork().unwrap();
        let event3 = test_event(3);
        experiment.record(&event3, &resolve, &store).unwrap();

        let changes = main.diff(&experiment, &resolve).unwrap();
        assert_eq!(changes.len(), 1);
        assert!(matches!(&changes[0], LedgerChange::Added(id) if *id == event3.id));

        let changes = experiment.diff(&main, &resolve).unwrap();
        assert_eq!(changes.len(), 1);
        assert!(matches!(&changes[0], LedgerChange::Removed(id) if *id == event3.id));
    }

    #[test]
    fn fork_shares_structure() {
        let (store, resolve) = memory_store();

        let chronicle = Chronicle::new();
        chronicle.record(&test_event(1), &resolve, &store).unwrap();

        let forked = chronicle.fork().unwrap();
        assert_eq!(chronicle.root().unwrap(), forked.root().unwrap());

        let changes = chronicle.diff(&forked, &resolve).unwrap();
        assert!(changes.is_empty());
    }
}
