//! EventLog — the append-only event persistence primitive.
//!
//! This is infrastructure, not a domain. It owns exactly one thing:
//! durable event persistence. Append, load, import, delete. It does
//! not run projections. It does not broadcast. It is the lowest layer.

use rusqlite::params;

use crate::*;

/// The append-only event store.
///
/// Pure persistence: append events, load them back, import from
/// external sources, delete transient entries. No projections,
/// no broadcasting — those are the bus's concern.
pub struct EventLog<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> EventLog<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
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
        let id = EventId::new();
        let data_json = serde_json::to_string(&event.data)?;
        let source_json = serde_json::to_string(&event.source)?;
        let event_type = event.data.event_type();
        let now = Timestamp::now();

        self.conn.execute(
            "INSERT INTO events (id, event_type, data, source, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id.to_string(), event_type, data_json, source_json, now.as_string()],
        )?;

        let sequence = self.conn.last_insert_rowid();

        Ok(StoredEvent::builder()
            .id(id)
            .sequence(sequence)
            .data(event.data.clone())
            .source(event.source)
            .created_at(now)
            .build())
    }

    /// Load all events in sequence order.
    pub fn load_all(&self) -> Result<Vec<StoredEvent>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, rowid, data, source, created_at FROM events ORDER BY rowid")?;

        let events = stmt
            .query_map([], |row| {
                let id_str: String = row.get(0)?;
                let data_str: String = row.get(2)?;
                let source_str: String = row.get(3)?;
                let created_at_str: String = row.get(4)?;
                Ok((id_str, row.get(1)?, data_str, source_str, created_at_str))
            })?
            .map(|result| {
                let (id_str, sequence, data_str, source_str, created_at_str) = result?;
                Ok(StoredEvent::builder()
                    .id(id_str.parse().unwrap_or_default())
                    .sequence(sequence)
                    .data(serde_json::from_str(&data_str).unwrap_or(Events::Unknown(
                        serde_json::from_str(&data_str).unwrap_or(serde_json::Value::Null),
                    )))
                    .source(serde_json::from_str(&source_str).unwrap_or_default())
                    .created_at(
                        Timestamp::parse_str(&created_at_str).unwrap_or_else(|_| Timestamp::now()),
                    )
                    .build())
            })
            .collect::<Result<Vec<_>, EventError>>()?;

        Ok(events)
    }

    /// Fetch events by ID. Returns all found events in sequence order;
    /// silently skips IDs that don't exist in the log.
    pub fn get_batch(&self, ids: &[EventId]) -> Result<Vec<StoredEvent>, EventError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders: Vec<String> = (1..=ids.len()).map(|i| format!("?{i}")).collect();
        let query = format!(
            "SELECT id, rowid, data, source, created_at FROM events WHERE id IN ({}) ORDER BY rowid",
            placeholders.join(",")
        );

        let mut stmt = self.conn.prepare(&query)?;
        let params: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        let param_refs: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();

        stmt.query_map(param_refs.as_slice(), |row| {
            let id_str: String = row.get(0)?;
            let data_str: String = row.get(2)?;
            let source_str: String = row.get(3)?;
            let created_at_str: String = row.get(4)?;
            Ok((id_str, row.get(1)?, data_str, source_str, created_at_str))
        })?
        .map(|result| {
            let (id_str, sequence, data_str, source_str, created_at_str) = result?;
            Ok(StoredEvent::builder()
                .id(id_str.parse().unwrap_or_default())
                .sequence(sequence)
                .data(serde_json::from_str(&data_str).unwrap_or(Events::Unknown(
                    serde_json::from_str(&data_str).unwrap_or(serde_json::Value::Null),
                )))
                .source(serde_json::from_str(&source_str).unwrap_or_default())
                .created_at(
                    Timestamp::parse_str(&created_at_str).unwrap_or_else(|_| Timestamp::now()),
                )
                .build())
        })
        .collect()
    }

    /// Import a single event without running projections. Idempotent.
    pub fn import(&self, event: &StoredEvent) -> Result<(), EventError> {
        let event_type = event.data.event_type();
        let data_json = serde_json::to_string(&event.data)?;
        let source_json = serde_json::to_string(&event.source)?;

        self.conn.execute(
            "INSERT OR IGNORE INTO events (id, event_type, data, source, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                event.id.to_string(),
                event_type,
                data_json,
                source_json,
                event.created_at.as_string(),
            ],
        )?;

        Ok(())
    }

    /// Delete a single event by ID.
    ///
    /// Rarely needed — the event log is append-only by design.
    /// Exists for administrative operations, not domain logic.
    pub fn delete(&self, event_id: &str) -> Result<(), EventError> {
        self.conn
            .execute("DELETE FROM events WHERE id = ?1", params![event_id])?;
        Ok(())
    }
}
