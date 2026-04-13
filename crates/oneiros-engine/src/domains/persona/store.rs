use rusqlite::params;

use crate::*;

/// Persona projection store — projection lifecycle, write operations, and sync read queries.
pub(crate) struct PersonaStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> PersonaStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Persona(persona_event) = &event.data {
            match persona_event {
                PersonaEvents::PersonaSet(persona) => self.set(persona)?,
                PersonaEvents::PersonaRemoved(removed) => self.remove(&removed.name)?,
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM personas", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS personas (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Sync read queries (for callers holding an open Connection) ──

    pub(crate) fn get(&self, name: &PersonaName) -> Result<Option<Persona>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM personas WHERE name = ?1")?;

        let result = stmt.query_row(params![name.to_string()], |row| {
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

    fn remove(&self, name: &PersonaName) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM personas WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }
}
