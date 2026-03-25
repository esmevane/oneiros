use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::*;

resource_id!(EventId);

/// A new event to be persisted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEvent {
    pub data: Events,
    pub source: Source,
}

/// A persisted event — the single envelope for storage, export, and import.
///
/// Serializes `created_at` as `created_at` internally and accepts
/// `timestamp` on deserialization for wire format compatibility.
#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct StoredEvent {
    pub id: EventId,
    pub sequence: i64,
    pub data: Events,
    #[serde(default)]
    pub source: Source,
    #[serde(alias = "timestamp")]
    pub created_at: Timestamp,
}
