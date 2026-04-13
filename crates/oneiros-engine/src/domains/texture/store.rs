use rusqlite::params;

use crate::*;

pub(crate) struct TextureStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> TextureStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Texture(texture_event) = &event.data {
            match texture_event {
                TextureEvents::TextureSet(texture) => self.set(texture)?,
                TextureEvents::TextureRemoved(removed) => self.remove(&removed.name)?,
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM textures", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS textures (
                name TEXT PRIMARY KEY,
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Sync read queries (for callers holding an open Connection) ──

    pub(crate) fn list(&self) -> Result<Vec<Texture>, EventError> {
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

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, texture: &Texture) -> Result<(), EventError> {
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

    fn remove(&self, name: &TextureName) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM textures WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }
}
