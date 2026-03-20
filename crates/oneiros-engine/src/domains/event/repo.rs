//! Event repository — SQLite-backed event persistence and projection runner.

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use uuid::Uuid;

use crate::events::{self, Events};
use crate::{EventError, NewEvent, Projection, StoredEvent};

/// Initialize the event store schema.
pub fn migrate(conn: &Connection) -> Result<(), EventError> {
    conn.execute_batch(
        "
        create table if not exists events (
            id text primary key,
            event_type text not null,
            data text not null,
            source text not null default '',
            created_at text not null
        );
        ",
    )?;

    Ok(())
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
) -> Result<StoredEvent, EventError> {
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
pub fn load_events(conn: &Connection) -> Result<Vec<StoredEvent>, EventError> {
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

/// Import a single event into the store without running projections.
///
/// Assigns a fresh sequence number. Uses INSERT OR IGNORE so that
/// re-importing the same event (by id) is idempotent.
pub fn import_event(conn: &Connection, event: &crate::ImportEvent) -> Result<(), EventError> {
    let (id, source, timestamp, data) = match event {
        crate::ImportEvent::Sourced {
            id,
            source,
            timestamp,
            data,
        } => (id, serde_json::to_string(source)?, timestamp.clone(), data),
        crate::ImportEvent::Unsourced { .. } => {
            return Err(EventError::Import("event has no source".into()));
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

/// Delete a single event by ID — used for transient events like BlobStored
/// that serve their purpose (carrying binary for export/import) and should
/// be cleaned up after the projection materializes the content.
pub fn delete_event(conn: &Connection, event_id: &str) -> Result<(), EventError> {
    conn.execute("DELETE FROM events WHERE id = ?1", params![event_id])?;
    Ok(())
}

/// Replay all events through projections (rebuild read models).
///
/// Returns the number of events replayed.
pub fn replay(conn: &Connection, projections: &[&[Projection]]) -> Result<usize, EventError> {
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
