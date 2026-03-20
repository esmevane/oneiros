use rusqlite::{Connection, params};

use crate::*;

/// Level read model — queries, projection handling, and lifecycle.
pub struct LevelRepo<'a> {
    conn: &'a Connection,
}

impl<'a> LevelRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Level(level_event) = &event.data {
            match level_event {
                LevelEvents::LevelSet(level) => self.set(level)?,
                LevelEvents::LevelRemoved(removed) => self.remove(&removed.name)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM levels", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS levels (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &LevelName) -> Result<Option<Level>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM levels WHERE name = ?1")?;

        let result = stmt.query_row(params![name.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        });

        match result {
            Ok((name, description, prompt)) => Ok(Some(
                Level::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Level>, EventError> {
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
