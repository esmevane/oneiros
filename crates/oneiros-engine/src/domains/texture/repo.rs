use rusqlite::{Connection, params};

use crate::store::{StoreError, StoredEvent};

use super::model::Texture;

/// Agent read model — queries, projection handling, and lifecycle.
pub struct TextureRepo<'a> {
    conn: &'a Connection,
}

impl<'a> TextureRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if event.event_type == "texture-set" {
            let texture: Texture = serde_json::from_value(event.data.clone())?;
            self.set(&texture)?;
        } else if event.event_type == "texture-removed"
            && let Some(name) = event.data.get("name").and_then(|v| v.as_str()) {
                self.remove(name)?;
            }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM textures", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
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

    pub fn get(&self, name: &str) -> Result<Option<Texture>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM textures WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
            Ok(Texture {
                name: row.get(0)?,
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

    pub fn list(&self) -> Result<Vec<Texture>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, description, prompt FROM textures ORDER BY name")?;

        let textures = stmt
            .query_map([], |row| {
                Ok(Texture {
                    name: row.get(0)?,
                    description: row.get(1)?,
                    prompt: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(textures)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn set(&self, texture: &Texture) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO textures (name, description, prompt) VALUES (?1, ?2, ?3)",
            params![texture.name, texture.description, texture.prompt],
        )?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<(), StoreError> {
        self.conn
            .execute("DELETE FROM textures WHERE name = ?1", params![name])?;
        Ok(())
    }
}
