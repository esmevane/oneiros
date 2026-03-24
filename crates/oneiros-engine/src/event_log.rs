//! EventLog — the append-only event persistence primitive.
//!
//! This is infrastructure, not a domain. It owns exactly one thing:
//! durable event persistence. Append, load, import, delete. It does
//! not run projections. It does not broadcast. It is the lowest layer.

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use uuid::Uuid;

use crate::events::{self, Events};
use crate::{EventError, ImportEvent, NewEvent, StoredEvent};

/// The append-only event store.
///
/// Pure persistence: append events, load them back, import from
/// external sources, delete transient entries. No projections,
/// no broadcasting — those are the bus's concern.
pub struct EventLog<'a> {
    conn: &'a Connection,
}

impl<'a> EventLog<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Create the events table.
    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
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

    /// Append a single event. Returns the stored form with sequence.
    pub fn append(&self, event: &NewEvent) -> Result<StoredEvent, EventError> {
        let id = Uuid::now_v7().to_string();
        let data_json = serde_json::to_string(&event.data)?;
        let source_json = serde_json::to_string(&event.source)?;
        let event_type = events::event_type(&event.data);
        let now = Utc::now();

        self.conn.execute(
            "INSERT INTO events (id, event_type, data, source, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, event_type, data_json, source_json, now.to_rfc3339()],
        )?;

        let sequence = self.conn.last_insert_rowid();

        Ok(StoredEvent {
            id,
            sequence,
            event_type,
            data: event.data.clone(),
            source: event.source,
            created_at: now,
        })
    }

    /// Load all events in sequence order.
    pub fn load_all(&self) -> Result<Vec<StoredEvent>, EventError> {
        let mut stmt = self.conn.prepare(
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

    /// Import a single event without running projections. Idempotent.
    pub fn import(&self, event: &ImportEvent) -> Result<(), EventError> {
        let (id, source, timestamp, data) = match event {
            ImportEvent::Sourced {
                id,
                source,
                timestamp,
                data,
            } => (id, serde_json::to_string(source)?, timestamp.clone(), data),
            ImportEvent::Unsourced { .. } => {
                return Err(EventError::Import("event has no source".into()));
            }
        };

        let event_type = data
            .get("type")
            .and_then(|t| t.as_str())
            .unwrap_or("unknown")
            .to_string();

        self.conn.execute(
            "INSERT OR IGNORE INTO events (id, event_type, data, source, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, event_type, data.to_string(), source, timestamp],
        )?;

        Ok(())
    }

    /// Delete a single event by ID — used for transient events like BlobStored.
    pub fn delete(&self, event_id: &str) -> Result<(), EventError> {
        self.conn
            .execute("DELETE FROM events WHERE id = ?1", params![event_id])?;
        Ok(())
    }
}
