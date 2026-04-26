use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::*;

resource_id!(EventId);

/// A new event to be persisted.
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
pub struct NewEvent {
    #[builder(into)]
    pub data: Events,
    #[builder(default)]
    pub source: Source,
}

/// A persisted event — the single envelope for storage, export, and import.
///
/// Serializes `created_at` as `created_at` internally and accepts
/// `timestamp` on deserialization for wire format compatibility.
#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct StoredEvent {
    pub id: EventId,
    #[serde(default)]
    pub sequence: i64,
    pub data: Event,
    #[serde(default)]
    pub source: Source,
    #[serde(alias = "timestamp")]
    pub created_at: Timestamp,
}
