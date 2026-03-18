use rusqlite::{Connection, params};

use crate::store::{StoreError, StoredEvent};

use super::model::Persona;

/// Agent read model — queries, projection handling, and lifecycle.
pub struct PersonaRepo<'a> {
    conn: &'a Connection,
}

impl<'a> PersonaRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if event.event_type == "persona-set" {
            let persona: Persona = serde_json::from_value(event.data.clone())?;
            self.set(&persona)?;
        } else if event.event_type == "persona-removed"
            && let Some(name) = event.data.get("name").and_then(|v| v.as_str()) {
                self.remove(name)?;
            }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM personas", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS personas (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &str) -> Result<Option<Persona>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM personas WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
            Ok(Persona {
                name: row.get(0)?,
                description: row.get(1)?,
                prompt: row.get(2)?,
            })
        });

        match result {
            Ok(persona) => Ok(Some(persona)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Persona>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM personas ORDER BY name")?;

        let personas = stmt
            .query_map([], |row| {
                Ok(Persona {
                    name: row.get(0)?,
                    description: row.get(1)?,
                    prompt: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(personas)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, persona: &Persona) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO personas (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![persona.name, persona.description, persona.prompt],
        )?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<(), StoreError> {
        self.conn
            .execute("DELETE FROM personas WHERE name = ?1", params![name])?;
        Ok(())
    }
}
