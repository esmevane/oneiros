use rusqlite::params;

use crate::*;

pub(crate) struct SliceStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SliceStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Slice(slice_event)) = &event.data {
            match slice_event {
                SliceEvents::SliceCreated(created) => self.insert(created)?,
                SliceEvents::SliceDeleted(deleted) => self.remove(deleted)?,
                SliceEvents::SliceMatched(matched) => self.increment(matched)?,
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM slices", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS slices (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                lens_expr TEXT NOT NULL,
                event_count INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            )",
        )?;
        Ok(())
    }

    fn insert(&self, created: &SliceCreated) -> Result<(), EventError> {
        let created = created.current()?;
        let slice = &created.slice;
        self.conn.execute(
            "INSERT INTO slices (id, name, lens_expr, event_count, created_at) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                slice.id.to_string(),
                slice.name.to_string(),
                slice.lens_expr,
                slice.event_count,
                slice.created_at.to_string(),
            ],
        )?;
        Ok(())
    }

    fn remove(&self, deleted: &SliceDeleted) -> Result<(), EventError> {
        let deleted = deleted.current()?;
        self.conn.execute(
            "DELETE FROM slices WHERE name = ?1",
            params![deleted.name.to_string()],
        )?;
        Ok(())
    }

    fn increment(&self, matched: &SliceMatched) -> Result<(), EventError> {
        let matched = matched.current()?;
        self.conn.execute(
            "UPDATE slices SET event_count = event_count + 1 WHERE name = ?1",
            params![matched.slice_name.to_string()],
        )?;
        Ok(())
    }
}
