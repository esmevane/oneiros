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
///
/// Two construction modes:
/// - `new(conn)` — standalone, events DB is the base connection.
///   Table references are unqualified (`events`).
/// - `attached(conn)` — the bookmark DB is the base connection and
///   the events DB is ATTACHed as `events`. Table references use
///   the `events.` schema qualifier.
pub struct EventLog<'a> {
    conn: &'a rusqlite::Connection,
    table: &'static str,
}

impl<'a> EventLog<'a> {
    /// Standalone mode — events DB is the base connection.
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self {
            conn,
            table: "events",
        }
    }

    /// ATTACH mode — bookmark DB is the base, events DB ATTACHed as `events`.
    pub fn attached(conn: &'a rusqlite::Connection) -> Self {
        Self {
            conn,
            table: "events.events",
        }
    }

    /// Create the events table.
    pub fn init(&self) -> Result<(), EventError> {
        self.conn.execute_batch(&format!(
            "create table if not exists {} (
                id         text primary key,
                event_type text not null,
                data       text not null,
                source     text not null default '',
                created_at text not null
            )",
            self.table,
        ))?;

        Ok(())
    }

    /// Append a single event. Returns the stored form with sequence.
    #[tracing::instrument(skip_all, fields(event_type = event.data.event_type()), err(Display))]
    pub fn append(&self, event: &NewEvent) -> Result<StoredEvent, EventError> {
        let id = EventId::new();
        let data_json = serde_json::to_string(&event.data)?;
        let source_json = serde_json::to_string(&event.source)?;
        let event_type = event.data.event_type();
        let now = Timestamp::now();

        self.conn.execute(
            &format!(
                "insert into {} (id, event_type, data, source, created_at) values (?1, ?2, ?3, ?4, ?5)",
                self.table,
            ),
            params![id.to_string(), event_type, data_json, source_json, now.as_string()],
        )?;

        let sequence = self.conn.last_insert_rowid();

        Ok(StoredEvent::builder()
            .id(id)
            .sequence(sequence)
            .data(Event::Known(event.data.clone()))
            .source(event.source)
            .created_at(now)
            .build())
    }

    /// Load all events in sequence order. Rows whose event type is not
    /// recognized are logged at warn and filtered out of the result.
    pub fn load_all(&self) -> Result<Vec<StoredEvent>, EventError> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT id, rowid, data, source, created_at FROM {} ORDER BY rowid",
            self.table,
        ))?;

        let rows = stmt
            .query_map([], |row| {
                let id_str: String = row.get(0)?;
                let data_str: String = row.get(2)?;
                let source_str: String = row.get(3)?;
                let created_at_str: String = row.get(4)?;
                Ok((id_str, row.get(1)?, data_str, source_str, created_at_str))
            })?
            .map(|result| -> Result<StoredEvent, EventError> {
                let (id_str, sequence, data_str, source_str, created_at_str) = result?;

                let id = id_str.parse().unwrap_or_default();
                let record = serde_json::from_str(&data_str)
                    .inspect_err(|error| {
                        tracing::warn!(%error, "skipping event row with malformed json");
                    })
                    .unwrap_or_default();

                Ok(StoredEvent::builder()
                    .id(id)
                    .sequence(sequence)
                    .data(record)
                    .source(serde_json::from_str(&source_str).unwrap_or_default())
                    .created_at(
                        Timestamp::parse_str(created_at_str).unwrap_or_else(|_| Timestamp::now()),
                    )
                    .build())
            });

        let mut events = Vec::new();
        for row in rows {
            let row = row?;
            if let Event::Known(_) = row.data {
                events.push(row);
            }
        }
        Ok(events)
    }

    /// Fetch events by ID. Returns all found events in sequence order;
    ///
    pub fn get_batch(&self, ids: &[EventId]) -> Result<Vec<StoredEvent>, EventError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders: Vec<String> = (1..=ids.len()).map(|i| format!("?{i}")).collect();
        let query = format!(
            "select id, rowid, data, source, created_at from {} where id in ({}) order by rowid",
            self.table,
            placeholders.join(","),
        );

        let mut stmt = self.conn.prepare(&query)?;
        let params: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        let param_refs: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();

        let rows = stmt
            .query_map(param_refs.as_slice(), |row| {
                let id_str: String = row.get(0)?;
                let data_str: String = row.get(2)?;
                let source_str: String = row.get(3)?;
                let created_at_str: String = row.get(4)?;
                Ok((id_str, row.get(1)?, data_str, source_str, created_at_str))
            })?
            .map(|result| -> Result<StoredEvent, EventError> {
                let (id_str, sequence, data_str, source_str, created_at_str) = result?;

                let id = id_str.parse().unwrap_or_default();
                let record = serde_json::from_str(&data_str)
                    .inspect_err(|error| {
                        tracing::warn!(%error, "skipping event row with malformed json");
                    })
                    .unwrap_or_default();

                Ok(StoredEvent::builder()
                    .id(id)
                    .sequence(sequence)
                    .data(record)
                    .source(serde_json::from_str(&source_str).unwrap_or_default())
                    .created_at(
                        Timestamp::parse_str(created_at_str).unwrap_or_else(|_| Timestamp::now()),
                    )
                    .build())
            });

        let mut events = Vec::new();

        for row in rows {
            let row = row?;
            if let Event::Known(_) = row.data {
                events.push(row)
            }
        }
        Ok(events)
    }

    /// Import a single event without running projections. Idempotent.
    pub fn import(&self, event: &StoredEvent) -> Result<(), EventError> {
        let event_type = event.data.event_type();
        let data_json = serde_json::to_string(&event.data)?;
        let source_json = serde_json::to_string(&event.source)?;

        self.conn.execute(
            &format!(
                "insert or ignore into {} (id, event_type, data, source, created_at) values (?1, ?2, ?3, ?4, ?5)",
                self.table,
            ),
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
        self.conn.execute(
            &format!("DELETE FROM {} WHERE id = ?1", self.table),
            params![event_id],
        )?;
        Ok(())
    }
}
