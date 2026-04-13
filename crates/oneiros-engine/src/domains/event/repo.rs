//! Event repository — domain-level event queries.
//!
//! Infrastructure-level event persistence lives in EventLog (event_log.rs).
//! This module is the event domain's read model — queries that serve the
//! user-facing event resource (list, show, filter by type/date/agent).
//!
//! The repo reads from the same `events` table that EventLog writes to.
//! It's a consumer of what the log persists, same as AgentRepo reads
//! from the agents projection table.

use rusqlite::params;

use crate::*;

pub(crate) struct EventRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> EventRepo<'a> {
    pub(crate) fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    /// Get a single event by ID.
    pub(crate) async fn get(&self, id: &str) -> Result<Option<StoredEvent>, EventError> {
        let db = self.context.db()?;
        let mut statement =
            db.prepare("select id, rowid, data, source, created_at from events where id = ?1")?;

        let result = statement.query_row(params![id], Self::row_to_event);

        match result {
            Ok(event) => Ok(Some(event)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// List events, optionally filtered by type.
    ///
    /// `event_type` is a filter on the DB column — it stays as a query
    /// parameter even though the field is no longer on `StoredEvent`.
    pub(crate) async fn list(&self, event_type: Option<&str>) -> Result<Vec<StoredEvent>, EventError> {
        let db = self.context.db()?;

        if let Some(event_type) = event_type {
            let mut statement = db.prepare(
                "select id, rowid, data, source, created_at from events where event_type = ?1 order by rowid",
            )?;

            let events = statement
                .query_map(params![event_type], Self::row_to_event)?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(events)
        } else {
            let mut statement = db
                .prepare("select id, rowid, data, source, created_at from events order by rowid")?;

            let events = statement
                .query_map([], Self::row_to_event)?
                .collect::<Result<Vec<_>, _>>()?;

            Ok(events)
        }
    }

    fn row_to_event(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredEvent> {
        let id_str: String = row.get(0)?;
        let data_str: String = row.get(2)?;
        let source_str: String = row.get(3)?;
        let created_at_str: String = row.get(4)?;
        Ok(StoredEvent::builder()
            .id(id_str.parse().unwrap_or_default())
            .sequence(row.get(1)?)
            .data(serde_json::from_str(&data_str).unwrap_or(Events::Unknown(
                serde_json::from_str(&data_str).unwrap_or(serde_json::Value::Null),
            )))
            .source(serde_json::from_str(&source_str).unwrap_or_default())
            .created_at(Timestamp::parse_str(&created_at_str).unwrap_or_else(|_| Timestamp::now()))
            .build())
    }
}
