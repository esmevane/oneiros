//! Event repository — domain-level event queries.
//!
//! Infrastructure-level event persistence lives in EventLog (event_log.rs).
//! This module is the event domain's read model — queries that serve the
//! user-facing event resource (list, show, filter by type/date/agent).
//!
//! The repo reads from the same `events` table that EventLog writes to.
//! It's a consumer of what the log persists, same as AgentRepo reads
//! from the agents projection table.

use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};

use crate::events::Events;
use crate::{EventError, Source, StoredEvent};

pub struct EventRepo<'a> {
    conn: &'a Connection,
}

impl<'a> EventRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Get a single event by ID.
    pub fn get(&self, id: &str) -> Result<Option<StoredEvent>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, rowid, event_type, data, source, created_at FROM events WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id], Self::row_to_event);

        match result {
            Ok(event) => Ok(Some(event)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// List events, optionally filtered by type.
    pub fn list(&self, event_type: Option<&str>) -> Result<Vec<StoredEvent>, EventError> {
        if let Some(event_type) = event_type {
            let mut stmt = self.conn.prepare(
                "SELECT id, rowid, event_type, data, source, created_at FROM events WHERE event_type = ?1 ORDER BY rowid",
            )?;
            let events = stmt
                .query_map(params![event_type], Self::row_to_event)?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(events)
        } else {
            let mut stmt = self.conn.prepare(
                "SELECT id, rowid, event_type, data, source, created_at FROM events ORDER BY rowid",
            )?;
            let events = stmt
                .query_map([], Self::row_to_event)?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(events)
        }
    }

    fn row_to_event(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredEvent> {
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
    }
}
