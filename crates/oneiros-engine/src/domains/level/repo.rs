use rusqlite::{Connection, params};

use crate::events::Events;
use crate::store::{StoreError, StoredEvent};

use super::events::LevelEvents;
use super::model::Level;

/// Agent read model — queries, projection handling, and lifecycle.
pub struct LevelRepo<'a> {
    conn: &'a Connection,
}

impl<'a> LevelRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if let Events::Level(level_event) = &event.data {
            match level_event {
                LevelEvents::LevelSet(level) => self.set(level)?,
                LevelEvents::LevelRemoved(removed) => self.remove(&removed.name)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM levels", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
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

    pub fn get(&self, name: &str) -> Result<Option<Level>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM levels WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
            Ok(Level {
                name: row.get(0)?,
                description: row.get(1)?,
                prompt: row.get(2)?,
            })
        });

        match result {
            Ok(level) => Ok(Some(level)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Level>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM levels ORDER BY name")?;

        let levels = stmt
            .query_map([], |row| {
                Ok(Level {
                    name: row.get(0)?,
                    description: row.get(1)?,
                    prompt: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(levels)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, level: &Level) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO levels (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![level.name, level.description, level.prompt],
        )?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<(), StoreError> {
        self.conn
            .execute("DELETE FROM levels WHERE name = ?1", params![name])?;
        Ok(())
    }
}
