use rusqlite::{Connection, params};

use crate::events::Events;
use crate::store::{StoreError, StoredEvent};

use super::events::SensationEvents;
use super::model::Sensation;

/// Agent read model — queries, projection handling, and lifecycle.
pub struct SensationRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SensationRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if let Events::Sensation(sensation_event) = &event.data {
            match sensation_event {
                SensationEvents::SensationSet(sensation) => self.set(sensation)?,
                SensationEvents::SensationRemoved(removed) => self.remove(&removed.name)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM sensations", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sensations (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &str) -> Result<Option<Sensation>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM sensations WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
            Ok(Sensation {
                name: row.get(0)?,
                description: row.get(1)?,
                prompt: row.get(2)?,
            })
        });

        match result {
            Ok(sensation) => Ok(Some(sensation)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Sensation>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM sensations ORDER BY name")?;

        let sensations = stmt
            .query_map([], |row| {
                Ok(Sensation {
                    name: row.get(0)?,
                    description: row.get(1)?,
                    prompt: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(sensations)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, sensation: &Sensation) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO sensations (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![sensation.name, sensation.description, sensation.prompt],
        )?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<(), StoreError> {
        self.conn
            .execute("DELETE FROM sensations WHERE name = ?1", params![name])?;
        Ok(())
    }
}
