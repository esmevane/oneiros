use rusqlite::params;

use crate::*;

pub struct TextureStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> TextureStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Texture(texture_event)) = &event.data {
            match texture_event {
                TextureEvents::TextureSet(setting) => self.set(setting)?,
                TextureEvents::TextureRemoved(removal) => self.remove(removal)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM textures", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS textures (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Texture>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM textures ORDER BY name")?;

        let textures = stmt
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
                Texture::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(textures)
    }

    fn set(&self, setting: &TextureSet) -> Result<(), EventError> {
        let texture = setting.current()?.texture;
        self.write_texture(&texture)
    }

    fn remove(&self, removal: &TextureRemoved) -> Result<(), EventError> {
        let name = removal.current()?.name;
        self.conn.execute(
            "DELETE FROM textures WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }

    fn write_texture(&self, texture: &Texture) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO textures (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                texture.name.to_string(),
                texture.description.to_string(),
                texture.prompt.to_string()
            ],
        )?;
        Ok(())
    }
}
