use serde::{Deserialize, Serialize};

use crate::*;

/// A lightweight summary of a bookmark's position at a point in its event
/// stream. Cheap to produce, cheap to compare: two checkpoints with identical
/// `cumulative_hash` values have seen the same events in the same order, which
/// makes sync negotiation O(1) instead of O(n).
///
/// Used by follows to track "where we left off" during collect, and by the
/// sync protocol to negotiate what to transfer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Checkpoint {
    /// Number of events applied within this view.
    pub sequence: u64,
    /// Rolling content hash over the events that produced this checkpoint.
    /// Two checkpoints with the same hash have seen identical events.
    pub cumulative_hash: ContentHash,
    /// The identifier of the most recent event seen, or `None` when empty.
    pub head: Option<EventId>,
    /// When this checkpoint was recorded.
    pub taken_at: Timestamp,
}

impl Checkpoint {
    /// An empty checkpoint — no events seen. Used at follow creation time
    /// before the first collect.
    pub fn empty() -> Self {
        Self {
            sequence: 0,
            cumulative_hash: ContentHash::default(),
            head: None,
            taken_at: Timestamp::now(),
        }
    }

    /// Whether this checkpoint represents "no events seen."
    pub fn is_empty(&self) -> bool {
        self.sequence == 0 && self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty_checkpoint_has_zero_sequence_and_no_head() {
        let cp = Checkpoint::empty();
        assert_eq!(cp.sequence, 0);
        assert!(cp.head.is_none());
        assert!(cp.is_empty());
    }

    #[test]
    fn checkpoint_roundtrip_through_serde() {
        let cp = Checkpoint {
            sequence: 42,
            cumulative_hash: ContentHash::new("abc123"),
            head: Some(EventId::new()),
            taken_at: Timestamp::now(),
        };
        let json = serde_json::to_string(&cp).unwrap();
        let decoded: Checkpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(cp, decoded);
    }

    #[test]
    fn non_empty_checkpoint_is_not_empty() {
        let cp = Checkpoint {
            sequence: 1,
            cumulative_hash: ContentHash::new("xyz"),
            head: Some(EventId::new()),
            taken_at: Timestamp::now(),
        };
        assert!(!cp.is_empty());
    }
}
