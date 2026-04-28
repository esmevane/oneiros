use rusqlite::params;

use crate::*;

pub struct LevelStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> LevelStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Level(level_event)) = &event.data {
            match level_event {
                LevelEvents::LevelSet(setting) => self.set(setting)?,
                LevelEvents::LevelRemoved(removal) => self.remove(removal)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM levels", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS levels (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Level>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM levels ORDER BY name")?;

        let levels = stmt
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
                Level::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(levels)
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
