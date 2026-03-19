use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Source;
use crate::events::Events;

/// A new event to be persisted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEvent {
    pub data: Events,
    pub source: Source,
}

/// A persisted event with full envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub id: String,
    pub sequence: i64,
    pub event_type: String,
    pub data: Events,
    pub source: Source,
    pub created_at: DateTime<Utc>,
}

/// An event in the portable export format.
///
/// Matches the legacy KnownEvent envelope shape so that JSONL files
/// are compatible between legacy and engine systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportEvent {
    pub id: String,
    pub sequence: i64,
    pub timestamp: String,
    pub source: Source,
    pub data: Events,
}

impl From<StoredEvent> for ExportEvent {
    fn from(e: StoredEvent) -> Self {
        Self {
            id: e.id,
            sequence: e.sequence,
            timestamp: e.created_at.to_rfc3339(),
            source: e.source,
            data: e.data,
        }
    }
}

/// An event being imported — permissive, accepts any JSON data.
///
/// Matches the legacy ImportEvent shape. The `data` field is raw JSON
/// so we can import events from newer or older versions without
/// requiring compile-time type knowledge.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportEvent {
    Sourced {
        id: String,
        source: Source,
        timestamp: String,
        data: serde_json::Value,
    },
    Unsourced {
        id: String,
        timestamp: String,
        data: serde_json::Value,
    },
}

impl ImportEvent {
    pub fn with_source(self, source: Source) -> Self {
        match self {
            ImportEvent::Unsourced {
                id,
                timestamp,
                data,
            } => ImportEvent::Sourced {
                id,
                source,
                timestamp,
                data,
            },
            sourced @ ImportEvent::Sourced { .. } => sourced,
        }
    }
}
