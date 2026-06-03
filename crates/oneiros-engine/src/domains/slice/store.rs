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
                SliceEvents::SliceMatched(matched) => self.record_match(matched)?,
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM slice_chronicle", [])?;
        self.conn.execute("DELETE FROM slices", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS slices (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                lens_expr TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS slice_chronicle (
                slice_name TEXT NOT NULL REFERENCES slices(name) ON DELETE CASCADE,
                event_id TEXT NOT NULL,
                matched_at TEXT NOT NULL,
                PRIMARY KEY (slice_name, event_id)
            );",
        )?;
        Ok(())
    }

    fn insert(&self, created: &SliceCreated) -> Result<(), EventError> {
        let created = created.current()?;
        let slice = &created.slice;
        self.conn.execute(
            "INSERT INTO slices (id, name, lens_expr, created_at) \
             VALUES (?1, ?2, ?3, ?4)",
            params![
                slice.id.to_string(),
                slice.name.to_string(),
                slice.lens_expr,
                slice.created_at.to_string(),
            ],
        )?;

        // Populate the chronicle with the initial retroactive matches.
        for event_id in &created.initial_event_ids {
            self.conn.execute(
                "INSERT OR IGNORE INTO slice_chronicle (slice_name, event_id, matched_at) \
                 VALUES (?1, ?2, ?3)",
                params![
                    slice.name.to_string(),
                    event_id.to_string(),
                    slice.created_at.to_string(),
                ],
            )?;
        }

        Ok(())
    }

    fn remove(&self, deleted: &SliceDeleted) -> Result<(), EventError> {
        let deleted = deleted.current()?;
        // CASCADE handles chronicle cleanup.
        self.conn.execute(
            "DELETE FROM slices WHERE name = ?1",
            params![deleted.name.to_string()],
        )?;
        Ok(())
    }

    fn record_match(&self, matched: &SliceMatched) -> Result<(), EventError> {
        let matched = matched.current()?;
        self.conn.execute(
            "INSERT OR IGNORE INTO slice_chronicle (slice_name, event_id, matched_at) \
             VALUES (?1, ?2, ?3)",
            params![
                matched.slice_name.to_string(),
                matched.matched_event_id.to_string(),
                Timestamp::now().to_string(),
            ],
        )?;
        Ok(())
    }
}
