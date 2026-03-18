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
    IdParseError,
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
    pub source: String,
}

/// A persisted event with sequence number and timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub sequence: i64,
    pub event_type: String,
    pub data: Events,
    pub source: String,
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
    let id = Uuid::now_v7();
    let data_json = serde_json::to_string(&event.data)?;
    let event_type = events::event_type(&event.data);
    let now = Utc::now();

    conn.execute(
        "INSERT INTO events (id, event_type, data, source, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            id.to_string(),
            event_type,
            data_json,
            event.source,
            now.to_rfc3339()
        ],
    )?;

    let sequence = conn.last_insert_rowid();

    let stored = StoredEvent {
        sequence,
        event_type,
        data: event.data.clone(),
        source: event.source.clone(),
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
    let mut stmt = conn
        .prepare("SELECT rowid, event_type, data, source, created_at FROM events ORDER BY rowid")?;

    let events = stmt
        .query_map([], |row| {
            let data_str: String = row.get(2)?;
            Ok(StoredEvent {
                sequence: row.get(0)?,
                event_type: row.get(1)?,
                data: serde_json::from_str(&data_str).unwrap_or(Events::Unknown(
                    serde_json::from_str(&data_str).unwrap_or(serde_json::Value::Null),
                )),
                source: row.get(3)?,
                created_at: {
                    let s: String = row.get(4)?;
                    DateTime::parse_from_rfc3339(&s)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_default()
                },
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(events)
}

/// Replay all events through projections (rebuild read models).
pub fn replay(conn: &Connection, projections: &[&[Projection]]) -> Result<(), StoreError> {
    // Reset all projections
    for group in projections {
        for projection in *group {
            (projection.reset)(conn)?;
        }
    }

    // Reload and replay
    let events = load_events(conn)?;
    for event in &events {
        for group in projections {
            for projection in *group {
                (projection.apply)(conn, event)?;
            }
        }
    }

    Ok(())
}
