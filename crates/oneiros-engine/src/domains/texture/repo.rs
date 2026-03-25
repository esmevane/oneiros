use rusqlite::{Connection, params};

use crate::*;

/// Texture read model — queries, projection handling, and lifecycle.
pub struct TextureRepo<'a> {
    conn: &'a Connection,
}

impl<'a> TextureRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Texture(texture_event) = &event.data {
            match texture_event {
                TextureEvents::TextureSet(texture) => self.set(texture)?,
                TextureEvents::TextureRemoved(removed) => self.remove(&removed.name)?,
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

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &TextureName) -> Result<Option<Texture>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM textures WHERE name = ?1")?;

        let result = stmt.query_row(params![name.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        });

        match result {
            Ok((name, description, prompt)) => Ok(Some(
                Texture::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
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
