use rusqlite::{Connection, params};

use crate::*;

/// Nature read model — queries, projection handling, and lifecycle.
pub struct NatureRepo<'a> {
    conn: &'a Connection,
}

impl<'a> NatureRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if let Events::Nature(nature_event) = &event.data {
            match nature_event {
                NatureEvents::NatureSet(nature) => self.set(nature)?,
                NatureEvents::NatureRemoved(removed) => self.remove(removed.name.as_str())?,
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
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &str) -> Result<Option<Nature>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM natures WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
            let name: String = row.get(0)?;
            Ok(Nature {
                name: NatureName::new(name),
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
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM natures ORDER BY name")?;

        let natures = stmt
            .query_map([], |row| {
                let name: String = row.get(0)?;
                Ok(Nature {
                    name: NatureName::new(name),
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
            params![nature.name.to_string(), nature.description, nature.prompt],
        )?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<(), StoreError> {
        self.conn
            .execute("DELETE FROM natures WHERE name = ?1", params![name])?;
        Ok(())
    }
}
