use rusqlite::{Connection, params};

use crate::*;

/// Persona read model — queries, projection handling, and lifecycle.
pub struct PersonaRepo<'a> {
    conn: &'a Connection,
}

impl<'a> PersonaRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Persona(persona_event) = &event.data {
            match persona_event {
                PersonaEvents::PersonaSet(persona) => self.set(persona)?,
                PersonaEvents::PersonaRemoved(removed) => self.remove(removed.name.as_str())?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM personas", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
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

    pub fn get(&self, name: &str) -> Result<Option<Persona>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM personas WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        });

        match result {
            Ok((name, description, prompt)) => Ok(Some(
                Persona::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Persona>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM personas ORDER BY name")?;

        let personas = stmt
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
                Persona::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(personas)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, persona: &Persona) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO personas (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                persona.name.to_string(),
                persona.description.to_string(),
                persona.prompt.to_string()
            ],
        )?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<(), EventError> {
        self.conn
            .execute("DELETE FROM personas WHERE name = ?1", params![name])?;
        Ok(())
    }
}
