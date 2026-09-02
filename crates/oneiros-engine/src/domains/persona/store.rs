use rusqlite::params;

use crate::*;

/// Persona projection store — projection lifecycle, write operations, and sync read queries.
pub(crate) struct PersonaStore<'a> {
    conn: &'a DbHandle,
}

impl<'a> PersonaStore<'a> {
    pub(crate) fn new(conn: &'a DbHandle) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Persona(persona_event)) = &event.data {
            match persona_event {
                PersonaEvents::PersonaSet(setting) => self.set(setting)?,
                PersonaEvents::PersonaRemoved(removal) => self.remove(removal)?,
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

    pub(crate) fn get(&self, name: &PersonaName) -> Result<Option<Persona>, EventError> {
        let raw = self.conn.query_row(
            "SELECT name, description, prompt FROM personas WHERE name = ?1",
            params![name.to_string()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match raw {
            Ok((name, description, prompt)) => Ok(Some(
                Persona::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(DbError::NotFound) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    fn set(&self, setting: &PersonaSet) -> Result<(), EventError> {
        let persona = setting.current()?.persona;
        self.write_persona(&persona)
    }

    fn remove(&self, removal: &PersonaRemoved) -> Result<(), EventError> {
        let name = removal.current()?.name;
        self.conn.execute(
            "DELETE FROM personas WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }

    fn write_persona(&self, persona: &Persona) -> Result<(), EventError> {
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
}
