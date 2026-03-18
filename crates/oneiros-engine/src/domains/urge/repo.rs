use rusqlite::{Connection, params};

use crate::*;

/// Urge read model — queries, projection handling, and lifecycle.
pub struct UrgeRepo<'a> {
    conn: &'a Connection,
}

impl<'a> UrgeRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if let Events::Urge(urge_event) = &event.data {
            match urge_event {
                UrgeEvents::UrgeSet(urge) => self.set(urge)?,
                UrgeEvents::UrgeRemoved(removed) => self.remove(removed.name.as_str())?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM urges", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS urges (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &str) -> Result<Option<Urge>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM urges WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
            let name: String = row.get(0)?;
            Ok(Urge {
                name: UrgeName::new(name),
                description: row.get(1)?,
                prompt: row.get(2)?,
            })
        });

        match result {
            Ok(urge) => Ok(Some(urge)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Urge>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM urges ORDER BY name")?;

        let urges = stmt
            .query_map([], |row| {
                let name: String = row.get(0)?;
                Ok(Urge {
                    name: UrgeName::new(name),
                    description: row.get(1)?,
                    prompt: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(urges)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, urge: &Urge) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO urges (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![urge.name.to_string(), urge.description, urge.prompt],
        )?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<(), StoreError> {
        self.conn
            .execute("DELETE FROM urges WHERE name = ?1", params![name])?;
        Ok(())
    }
}
