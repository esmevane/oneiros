use rusqlite::params;

use crate::*;

pub struct NatureStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> NatureStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Nature(nature_event)) = &event.data {
            match nature_event {
                NatureEvents::NatureSet(setting) => self.set(setting)?,
                NatureEvents::NatureRemoved(removal) => self.remove(removal)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM natures", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS natures (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Nature>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM natures ORDER BY name")?;

        let natures = stmt
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
                Nature::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(natures)
    }

    fn set(&self, setting: &NatureSet) -> Result<(), EventError> {
        let nature = setting.current()?.nature;
        self.write_nature(&nature)
    }

    fn remove(&self, removal: &NatureRemoved) -> Result<(), EventError> {
        let name = removal.current()?.name;
        self.conn.execute(
            "DELETE FROM natures WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }

    fn write_nature(&self, nature: &Nature) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO natures (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                nature.name.to_string(),
                nature.description.to_string(),
                nature.prompt.to_string()
            ],
        )?;
        Ok(())
    }
}
