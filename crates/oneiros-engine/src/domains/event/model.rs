use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::*;

resource_id!(EventId);

/// A new event to be persisted.
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct NewEvent {
    #[builder(into)]
    pub(crate) data: Events,
    #[builder(default)]
    pub(crate) source: Source,
}

/// A persisted event — the single envelope for storage, export, and import.
///
/// Serializes `created_at` as `created_at` internally and accepts
/// `timestamp` on deserialization for wire format compatibility.
#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub(crate) struct StoredEvent {
    pub(crate) id: EventId,
    #[serde(default)]
    pub(crate) sequence: i64,
    pub(crate) data: Events,
    #[serde(default)]
    pub(crate) source: Source,
    #[serde(alias = "timestamp")]
    pub(crate) created_at: Timestamp,
}
