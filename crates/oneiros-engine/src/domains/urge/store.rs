use rusqlite::params;

use crate::*;

pub(crate) struct UrgeStore<'a> {
    conn: &'a DbHandle,
}

impl<'a> UrgeStore<'a> {
    pub(crate) fn new(conn: &'a DbHandle) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Urge(urge_event)) = &event.data {
            match urge_event {
                UrgeEvents::UrgeSet(setting) => self.set(setting)?,
                UrgeEvents::UrgeRemoved(removal) => self.remove(removal)?,
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM urges", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS urges (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    pub(crate) fn list(&self) -> Result<Vec<Urge>, EventError> {
        let raw: Vec<(String, String, String)> = self.conn.query_map(
            "SELECT name, description, prompt FROM urges ORDER BY name",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;

        let urges = raw
            .into_iter()
            .map(|(name, description, prompt)| {
                Urge::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(urges)
    }

    fn set(&self, setting: &UrgeSet) -> Result<(), EventError> {
        let urge = setting.current()?.urge;
        self.write_urge(&urge)
    }

    fn remove(&self, removal: &UrgeRemoved) -> Result<(), EventError> {
        let name = removal.current()?.name;
        self.conn.execute(
            "DELETE FROM urges WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }

    fn write_urge(&self, urge: &Urge) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO urges (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                urge.name.to_string(),
                urge.description.to_string(),
                urge.prompt.to_string()
            ],
        )?;
        Ok(())
    }
}
