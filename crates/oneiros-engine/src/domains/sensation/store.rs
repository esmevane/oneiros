use rusqlite::params;

use crate::*;

pub(crate) struct SensationStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SensationStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Sensation(sensation_event) = &event.data {
            match sensation_event {
                SensationEvents::SensationSet(sensation) => self.set(sensation)?,
                SensationEvents::SensationRemoved(removed) => self.remove(&removed.name)?,
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM sensations", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sensations (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Sync read queries (for callers holding an open Connection) ──

    pub(crate) fn list(&self) -> Result<Vec<Sensation>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM sensations ORDER BY name")?;

        let sensations = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })?
            .collect::<Result<Vec<(String, String, String)>, _>>()?
            .into_iter()
            .map(|(name, description, prompt)| {
                Sensation::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(sensations)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, sensation: &Sensation) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO sensations (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                sensation.name.to_string(),
                sensation.description.to_string(),
                sensation.prompt.to_string()
            ],
        )?;
        Ok(())
    }

    fn remove(&self, name: &SensationName) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM sensations WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }
}
