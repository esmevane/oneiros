//! Ledger — a content-addressed HAMT for tracking sets of event IDs.
//!
//! The ledger is the pure algorithmic layer behind chronicles. It implements
//! a Hash Array Mapped Trie (HAMT) where each node is content-addressed:
//! serialize → hash → store. Structural sharing means unchanged subtrees
//! keep their existing hashes, making forking free and diffing proportional
//! to the number of differences, not the size of either tree.
//!
//! The ledger is stateless — all operations take a root hash and return
//! a new root hash. Persistence is handled by the caller (ChronicleStore).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::*;

/// Maximum entries in a leaf node before it splits into an interior node.
const MAX_LEAF_ENTRIES: usize = 16;

/// A HAMT node — either a leaf containing event entries, or an interior
/// node whose children are keyed by nibble (4-bit prefix of the event ID
/// at the current trie depth).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum LedgerNode {
    Leaf {
        entries: BTreeMap<String, ContentHash>,
    },
    Interior {
        children: BTreeMap<u8, ContentHash>,
    },
}

/// The result of diffing two ledger roots.
#[derive(Debug, Clone)]
pub(crate) enum LedgerChange {
    Added(EventId),
    Removed(EventId),
}

/// Extract the nibble (4-bit value, 0..16) at the given depth from an event ID string.
/// Uses the hex characters of the ID's string representation.
fn nibble_at(id: &str, depth: usize) -> u8 {
    id.as_bytes()
        .get(depth)
        .map(|b| match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => 10 + b - b'a',
            b'A'..=b'F' => 10 + b - b'A',
            _ => 0,
        })
        .unwrap_or(0)
}

/// Operations on a content-addressed ledger.
///
/// The `resolve` and `store` closures abstract over the persistence layer,
/// keeping the ledger algorithm pure.
pub(crate) struct Ledger;

impl Ledger {
    /// Record an event ID in the ledger. Returns the new root hash.
    pub(crate) fn record(
        root: Option<&ContentHash>,
        event_id: &EventId,
        event_hash: ContentHash,
        resolve: &impl Fn(&ContentHash) -> Option<LedgerNode>,
        store: &impl Fn(&LedgerNode) -> ContentHash,
    ) -> ContentHash {
        let key = event_id.to_string();
        match root {
            None => {
                let mut entries = BTreeMap::new();
                entries.insert(key, event_hash);
                store(&LedgerNode::Leaf { entries })
            }
            Some(hash) => Self::record_at(hash, &key, event_hash, 0, resolve, store),
        }
    }

    fn record_at(
        node_hash: &ContentHash,
        key: &str,
        value: ContentHash,
        depth: usize,
        resolve: &impl Fn(&ContentHash) -> Option<LedgerNode>,
        store: &impl Fn(&LedgerNode) -> ContentHash,
    ) -> ContentHash {
        let node = match resolve(node_hash) {
            Some(n) => n,
            None => {
                return store(&LedgerNode::Leaf {
                    entries: BTreeMap::from([(key.to_string(), value)]),
                });
            }
        };

        match node {
            LedgerNode::Leaf { mut entries } => {
                entries.insert(key.to_string(), value);
                if entries.len() <= MAX_LEAF_ENTRIES {
                    store(&LedgerNode::Leaf { entries })
                } else {
                    Self::split(&entries, depth, store)
                }
            }
            LedgerNode::Interior { mut children } => {
                let nibble = nibble_at(key, depth);
                let child = children.get(&nibble).cloned();
                let new_child = Self::record(
                    child.as_ref(),
                    &key.parse().unwrap_or_default(),
                    value,
                    resolve,
                    store,
                );
                children.insert(nibble, new_child);
                store(&LedgerNode::Interior { children })
            }
        }
    }

    fn split(
        entries: &BTreeMap<String, ContentHash>,
        depth: usize,
        store: &impl Fn(&LedgerNode) -> ContentHash,
    ) -> ContentHash {
        let mut buckets: BTreeMap<u8, BTreeMap<String, ContentHash>> = BTreeMap::new();
        for (key, value) in entries {
            let nibble = nibble_at(key, depth);
            buckets
                .entry(nibble)
                .or_default()
                .insert(key.clone(), value.clone());
        }
        let mut children = BTreeMap::new();
        for (nibble, bucket_entries) in buckets {
            let child_hash = store(&LedgerNode::Leaf {
                entries: bucket_entries,
            });
            children.insert(nibble, child_hash);
        }
        store(&LedgerNode::Interior { children })
    }

    /// Diff two ledger roots. Produces a minimal set of changes.
    /// Cost is proportional to the number of differences, not tree size.
    pub(crate) fn diff(
        a: Option<&ContentHash>,
        b: Option<&ContentHash>,
        resolve: &impl Fn(&ContentHash) -> Option<LedgerNode>,
    ) -> Vec<LedgerChange> {
        if a == b {
            return vec![];
        }
        match (a, b) {
            (None, None) => vec![],
            (None, Some(b_hash)) => {
                let entries = Self::collect_all(b_hash, resolve);
                entries
                    .into_keys()
                    .filter_map(|k| k.parse().ok())
                    .map(LedgerChange::Added)
                    .collect()
            }
            (Some(a_hash), None) => {
                let entries = Self::collect_all(a_hash, resolve);
                entries
                    .into_keys()
                    .filter_map(|k| k.parse().ok())
                    .map(LedgerChange::Removed)
                    .collect()
            }
            (Some(a_hash), Some(b_hash)) => Self::diff_nodes(a_hash, b_hash, resolve),
        }
    }

    fn diff_nodes(
        a_hash: &ContentHash,
        b_hash: &ContentHash,
        resolve: &impl Fn(&ContentHash) -> Option<LedgerNode>,
    ) -> Vec<LedgerChange> {
        if a_hash == b_hash {
            return vec![];
        }

        let a_node = resolve(a_hash);
        let b_node = resolve(b_hash);

        match (a_node, b_node) {
            (
                Some(LedgerNode::Leaf { entries: a_entries }),
                Some(LedgerNode::Leaf { entries: b_entries }),
            ) => Self::diff_entries(&a_entries, &b_entries),
            (
                Some(LedgerNode::Interior {
                    children: a_children,
                }),
                Some(LedgerNode::Interior {
                    children: b_children,
                }),
            ) => {
                let mut changes = Vec::new();
                let all_nibbles: std::collections::BTreeSet<u8> = a_children
                    .keys()
                    .chain(b_children.keys())
                    .copied()
                    .collect();
                for nibble in all_nibbles {
                    let a_child = a_children.get(&nibble);
                    let b_child = b_children.get(&nibble);
                    changes.extend(Self::diff(a_child, b_child, resolve));
                }
                changes
            }
            _ => {
                let a_entries = a_hash
                    .to_string()
                    .is_empty()
                    .then(BTreeMap::new)
                    .unwrap_or_else(|| Self::collect_all(a_hash, resolve));
                let b_entries = b_hash
                    .to_string()
                    .is_empty()
                    .then(BTreeMap::new)
                    .unwrap_or_else(|| Self::collect_all(b_hash, resolve));
                Self::diff_entries(&a_entries, &b_entries)
            }
        }
    }

    fn diff_entries(
        a: &BTreeMap<String, ContentHash>,
        b: &BTreeMap<String, ContentHash>,
    ) -> Vec<LedgerChange> {
        let mut changes = Vec::new();

        for (id, a_hash) in a {
            match b.get(id) {
                None => {
                    if let Ok(event_id) = id.parse() {
                        changes.push(LedgerChange::Removed(event_id));
                    }
                }
                Some(b_hash) if b_hash != a_hash => {
                    if let Ok(event_id) = id.parse() {
                        changes.push(LedgerChange::Added(event_id));
                    }
                }
                _ => {}
            }
        }

        for id in b.keys() {
            if !a.contains_key(id)
                && let Ok(event_id) = id.parse()
            {
                changes.push(LedgerChange::Added(event_id));
            }
        }

        changes
    }

    /// Collect all event ID strings reachable from a root.
    pub(crate) fn collect_all_ids(
        root: Option<&ContentHash>,
        resolve: &impl Fn(&ContentHash) -> Option<LedgerNode>,
    ) -> std::collections::HashSet<String> {
        match root {
            None => std::collections::HashSet::new(),
            Some(hash) => Self::collect_all(hash, resolve).into_keys().collect(),
        }
    }

    pub(crate) fn collect_all(
        node_hash: &ContentHash,
        resolve: &impl Fn(&ContentHash) -> Option<LedgerNode>,
    ) -> BTreeMap<String, ContentHash> {
        match resolve(node_hash) {
            Some(LedgerNode::Leaf { entries }) => entries,
            Some(LedgerNode::Interior { children }) => {
                let mut all = BTreeMap::new();
                for (_nibble, child_hash) in children {
                    all.extend(Self::collect_all(&child_hash, resolve));
                }
                all
            }
            None => BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// In-memory store for testing.
    fn memory_store() -> (
        impl Fn(&LedgerNode) -> ContentHash,
        impl Fn(&ContentHash) -> Option<LedgerNode>,
    ) {
        let store: std::sync::Arc<Mutex<HashMap<String, LedgerNode>>> =
            std::sync::Arc::new(Mutex::new(HashMap::new()));

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

    #[test]
    fn record_and_diff_empty() {
        let (store, resolve) = memory_store();

        let event_id = EventId::new();
        let event_hash = ContentHash::compute(b"event-data");

        let root = Ledger::record(None, &event_id, event_hash, &resolve, &store);

        let changes = Ledger::diff(None, Some(&root), &resolve);
        assert_eq!(changes.len(), 1);
        assert!(matches!(&changes[0], LedgerChange::Added(id) if *id == event_id));
    }

    #[test]
    fn diff_identical_roots() {
        let (store, resolve) = memory_store();

        let event_id = EventId::new();
        let event_hash = ContentHash::compute(b"event-data");

        let root = Ledger::record(None, &event_id, event_hash, &resolve, &store);

        let changes = Ledger::diff(Some(&root), Some(&root), &resolve);
        assert!(changes.is_empty());
    }

    #[test]
    fn diff_divergent_roots() {
        let (store, resolve) = memory_store();

        let shared_id = EventId::new();
        let shared_hash = ContentHash::compute(b"shared");

        // Both branches start with the same event
        let root_a = Ledger::record(None, &shared_id, shared_hash.clone(), &resolve, &store);
        let root_b = Ledger::record(None, &shared_id, shared_hash, &resolve, &store);

        // Branch A gets one more event
        let extra_id = EventId::new();
        let extra_hash = ContentHash::compute(b"extra");
        let root_a = Ledger::record(Some(&root_a), &extra_id, extra_hash, &resolve, &store);

        // Diff: A has one event that B doesn't
        let changes = Ledger::diff(Some(&root_b), Some(&root_a), &resolve);
        assert_eq!(changes.len(), 1);
        assert!(matches!(&changes[0], LedgerChange::Added(id) if *id == extra_id));
    }

    #[test]
    fn fork_is_free() {
        let (store, resolve) = memory_store();

        let id1 = EventId::new();
        let id2 = EventId::new();

        let root = Ledger::record(None, &id1, ContentHash::compute(b"1"), &resolve, &store);

        // "Fork" by cloning the root hash
        let fork_root = root.clone();

        // Add to the fork
        let fork_root = Ledger::record(
            Some(&fork_root),
            &id2,
            ContentHash::compute(b"2"),
            &resolve,
            &store,
        );

        // Original unchanged
        let changes = Ledger::diff(Some(&root), Some(&fork_root), &resolve);
        assert_eq!(changes.len(), 1);
        assert!(matches!(&changes[0], LedgerChange::Added(id) if *id == id2));
    }
}
