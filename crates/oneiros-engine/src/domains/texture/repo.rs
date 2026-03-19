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
                TextureEvents::TextureRemoved(removed) => self.remove(removed.name.as_str())?,
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

    pub fn get(&self, name: &str) -> Result<Option<Texture>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM textures WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
            let name: String = row.get(0)?;
            Ok(Texture {
                name: TextureName::new(name),
                description: row.get(1)?,
                prompt: row.get(2)?,
            })
        });

        match result {
            Ok(texture) => Ok(Some(texture)),
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
                let name: String = row.get(0)?;
                Ok(Texture {
                    name: TextureName::new(name),
                    description: row.get(1)?,
                    prompt: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(textures)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, texture: &Texture) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO textures (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![
                texture.name.to_string(),
                texture.description,
                texture.prompt
            ],
        )?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<(), EventError> {
        self.conn
            .execute("DELETE FROM textures WHERE name = ?1", params![name])?;
        Ok(())
    }
}
