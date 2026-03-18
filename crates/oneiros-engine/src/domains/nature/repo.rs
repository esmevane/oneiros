use rusqlite::{Connection, params};

use crate::store::{StoredEvent, StoreError};

use super::model::Nature;

/// Agent read model — queries, projection handling, and lifecycle.
pub struct NatureRepo<'a> {
    conn: &'a Connection,
}

impl<'a> NatureRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if event.event_type == "nature-set" {
            let nature: Nature = serde_json::from_value(event.data.clone())?;
            self.set(&nature)?;
        } else if event.event_type == "nature-removed" {
            if let Some(name) = event.data.get("name").and_then(|v| v.as_str()) {
                self.remove(name)?;
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM natures", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS natures (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )"
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &str) -> Result<Option<Nature>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT name, description, prompt FROM natures WHERE name = ?1",
        )?;

        let result = stmt.query_row(params![name], |row| {
            Ok(Nature {
                name: row.get(0)?,
                description: row.get(1)?,
                prompt: row.get(2)?,
            })
        });

        match result {
            Ok(nature) => Ok(Some(nature)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Nature>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT name, description, prompt FROM natures ORDER BY name",
        )?;

        let natures = stmt.query_map([], |row| {
            Ok(Nature {
                name: row.get(0)?,
                description: row.get(1)?,
                prompt: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(natures)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, nature: &Nature) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO natures (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![nature.name, nature.description, nature.prompt],
        )?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM natures WHERE name = ?1", params![name])?;
        Ok(())
    }
}
