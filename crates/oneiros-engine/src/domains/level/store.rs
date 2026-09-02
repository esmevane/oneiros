use rusqlite::params;

use crate::*;

pub(crate) struct LevelStore<'a> {
    conn: &'a DbHandle,
}

impl<'a> LevelStore<'a> {
    pub(crate) fn new(conn: &'a DbHandle) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Level(level_event)) = &event.data {
            match level_event {
                LevelEvents::LevelSet(setting) => self.set(setting)?,
                LevelEvents::LevelRemoved(removal) => self.remove(removal)?,
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM levels", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS levels (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    pub(crate) fn list(&self) -> Result<Vec<Level>, EventError> {
        let tuples: Vec<(String, String, String)> = self.conn.query_map(
            "SELECT name, description, prompt FROM levels ORDER BY name",
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
                Level::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect())
    }

    fn set(&self, setting: &LevelSet) -> Result<(), EventError> {
        let level = setting.current()?.level;
        self.write_level(&level)
    }

    fn remove(&self, removal: &LevelRemoved) -> Result<(), EventError> {
        let name = removal.current()?.name;
        self.conn.execute(
            "DELETE FROM levels WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }

    fn write_level(&self, level: &Level) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO levels (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                level.name.to_string(),
                level.description.to_string(),
                level.prompt.to_string()
            ],
        )?;
        Ok(())
    }
}
