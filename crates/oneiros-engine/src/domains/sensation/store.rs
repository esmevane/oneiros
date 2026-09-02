use rusqlite::params;

use crate::*;

pub(crate) struct SensationStore<'a> {
    conn: &'a DbHandle,
}

impl<'a> SensationStore<'a> {
    pub(crate) fn new(conn: &'a DbHandle) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Sensation(sensation_event)) = &event.data {
            match sensation_event {
                SensationEvents::SensationSet(setting) => self.set(setting)?,
                SensationEvents::SensationRemoved(removal) => self.remove(removal)?,
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM sensations", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sensations (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    pub(crate) fn list(&self) -> Result<Vec<Sensation>, EventError> {
        let tuples: Vec<(String, String, String)> = self.conn.query_map(
            "SELECT name, description, prompt FROM sensations ORDER BY name",
            [],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        )?;

        Ok(tuples
            .into_iter()
            .map(|(name, description, prompt)| {
                Sensation::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect())
    }

    fn set(&self, setting: &SensationSet) -> Result<(), EventError> {
        let sensation = setting.current()?.sensation;
        self.write_sensation(&sensation)
    }

    fn remove(&self, removal: &SensationRemoved) -> Result<(), EventError> {
        let name = removal.current()?.name;
        self.conn.execute(
            "DELETE FROM sensations WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }

    fn write_sensation(&self, sensation: &Sensation) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO sensations (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                sensation.name.to_string(),
                sensation.description.to_string(),
                sensation.prompt.to_string()
            ],
        )?;
        Ok(())
    }
}
