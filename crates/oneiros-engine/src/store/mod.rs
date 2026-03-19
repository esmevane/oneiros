//! Event store — SQLite-backed event persistence and projection runner.
//!
//! This is the universal infrastructure. It knows nothing about domains.
//! It stores events as JSON blobs and runs projections after each write.

mod schema;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    IdParseError, Source,
    events::{self, Events},
};

pub use schema::initialize;

/// A projection — transforms events into read model state.
///
/// Each domain declares its projections. The event store runs them
/// after persisting each event. The projection's apply function
/// receives the database connection and the persisted event.
pub struct Projection {
    pub name: &'static str,
    pub apply: fn(&Connection, &StoredEvent) -> Result<(), StoreError>,
    pub reset: fn(&Connection) -> Result<(), StoreError>,
}

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

/// Event store errors.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    IdParse(#[from] IdParseError),

    #[error("Import error: {0}")]
    Import(String),
}

/// Append an event to the store and run projections.
///
/// This is the single write path. All domain services go through this.
/// Projections run synchronously in the same transaction — the read
/// model is always consistent with the event log.
pub fn log_event(
    conn: &Connection,
    event: &NewEvent,
    projections: &[&[Projection]],
) -> Result<StoredEvent, StoreError> {
    let id = Uuid::now_v7().to_string();
    let data_json = serde_json::to_string(&event.data)?;
    let source_json = serde_json::to_string(&event.source)?;
    let event_type = events::event_type(&event.data);
    let now = Utc::now();

    conn.execute(
        "INSERT INTO events (id, event_type, data, source, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, event_type, data_json, source_json, now.to_rfc3339()],
    )?;

    let sequence = conn.last_insert_rowid();

    let stored = StoredEvent {
        id,
        sequence,
        event_type,
        data: event.data.clone(),
        source: event.source,
        created_at: now,
    };

    // Run all projections
    for group in projections {
        for projection in *group {
            (projection.apply)(conn, &stored)?;
        }
    }

    Ok(stored)
}

/// Load all events from the store.
pub fn load_events(conn: &Connection) -> Result<Vec<StoredEvent>, StoreError> {
    let mut stmt = conn.prepare(
        "SELECT id, rowid, event_type, data, source, created_at FROM events ORDER BY rowid",
    )?;

    let events = stmt
        .query_map([], |row| {
            let data_str: String = row.get(3)?;
            let source_str: String = row.get(4)?;
            Ok(StoredEvent {
                id: row.get(0)?,
                sequence: row.get(1)?,
                event_type: row.get(2)?,
                data: serde_json::from_str(&data_str).unwrap_or(Events::Unknown(
                    serde_json::from_str(&data_str).unwrap_or(serde_json::Value::Null),
                )),
                source: serde_json::from_str(&source_str).unwrap_or_default(),
                created_at: {
                    let s: String = row.get(5)?;
                    DateTime::parse_from_rfc3339(&s)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_default()
                },
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(events)
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

/// Import a single event into the store without running projections.
///
/// Assigns a fresh sequence number. Uses INSERT OR IGNORE so that
/// re-importing the same event (by id) is idempotent.
pub fn import_event(conn: &Connection, event: &ImportEvent) -> Result<(), StoreError> {
    let (id, source, timestamp, data) = match event {
        ImportEvent::Sourced {
            id,
            source,
            timestamp,
            data,
        } => (id, serde_json::to_string(source)?, timestamp.clone(), data),
        ImportEvent::Unsourced { .. } => {
            return Err(StoreError::Import("event has no source".into()));
        }
    };

    let event_type = data
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("unknown")
        .to_string();

    conn.execute(
        "INSERT OR IGNORE INTO events (id, event_type, data, source, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, event_type, data.to_string(), source, timestamp],
    )?;

    Ok(())
}

/// Replay all events through projections (rebuild read models).
///
/// Returns the number of events replayed.
pub fn replay(conn: &Connection, projections: &[&[Projection]]) -> Result<usize, StoreError> {
    // Reset all projections
    for group in projections {
        for projection in *group {
            (projection.reset)(conn)?;
        }
    }

    // Reload and replay
    let events = load_events(conn)?;
    let count = events.len();
    for event in &events {
        for group in projections {
            for projection in *group {
                (projection.apply)(conn, event)?;
            }
        }
    }

    Ok(count)
}
