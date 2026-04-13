use rusqlite::params;

use crate::*;

pub(crate) struct LevelStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> LevelStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Level(level_event) = &event.data {
            match level_event {
                LevelEvents::LevelSet(level) => self.set(level)?,
                LevelEvents::LevelRemoved(removed) => self.remove(&removed.name)?,
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM levels", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS levels (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Sync read queries (for callers holding an open Connection) ──

    pub(crate) fn list(&self) -> Result<Vec<Level>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM levels ORDER BY name")?;

        let levels = stmt
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
                Level::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(levels)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, level: &Level) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO levels (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                level.name.to_string(),
                level.description.to_string(),
                level.prompt.to_string()
            ],
        )?;
        Ok(())
    }

    fn remove(&self, name: &LevelName) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM levels WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }
}
