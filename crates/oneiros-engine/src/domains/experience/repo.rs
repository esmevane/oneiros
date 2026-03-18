use rusqlite::{Connection, params};

use crate::store::{StoreError, StoredEvent};

use super::model::Experience;

/// Experience read model — queries, projection handling, and lifecycle.
pub struct ExperienceRepo<'a> {
    conn: &'a Connection,
}

impl<'a> ExperienceRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        match event.event_type.as_str() {
            "experience-created" => {
                let experience: Experience = serde_json::from_value(event.data.clone())?;
                self.insert(&experience)?;
            }
            "experience-description-updated" => {
                if let (Some(id), Some(description)) = (
                    event.data.get("id").and_then(|v| v.as_str()),
                    event.data.get("description").and_then(|v| v.as_str()),
                ) {
                    self.update_description(id, description)?;
                }
            }
            "experience-sensation-updated" => {
                if let (Some(id), Some(sensation)) = (
                    event.data.get("id").and_then(|v| v.as_str()),
                    event.data.get("sensation").and_then(|v| v.as_str()),
                ) {
                    self.update_sensation(id, sensation)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM experiences", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS experiences (
                id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                sensation TEXT NOT NULL DEFAULT '',
                description TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, id: &str) -> Result<Option<Experience>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, sensation, description, created_at
             FROM experiences WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id], |row| {
            Ok(Experience {
                id: row.get(0)?,
                agent_id: row.get(1)?,
                sensation: row.get(2)?,
                description: row.get(3)?,
                created_at: row.get(4)?,
            })
        });

        match result {
            Ok(e) => Ok(Some(e)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self, agent: Option<&str>) -> Result<Vec<Experience>, StoreError> {
        let mut stmt = match agent {
            Some(_) => self.conn.prepare(
                "SELECT id, agent_id, sensation, description, created_at
                 FROM experiences WHERE agent_id = ?1 ORDER BY created_at",
            )?,
            None => self.conn.prepare(
                "SELECT id, agent_id, sensation, description, created_at
                 FROM experiences ORDER BY created_at",
            )?,
        };

        let map_row = |row: &rusqlite::Row<'_>| {
            Ok(Experience {
                id: row.get(0)?,
                agent_id: row.get(1)?,
                sensation: row.get(2)?,
                description: row.get(3)?,
                created_at: row.get(4)?,
            })
        };

        let experiences = match agent {
            Some(a) => stmt.query_map(params![a], map_row),
            None => stmt.query_map([], map_row),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(experiences)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn insert(&self, experience: &Experience) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO experiences (id, agent_id, sensation, description, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                experience.id,
                experience.agent_id,
                experience.sensation,
                experience.description,
                experience.created_at,
            ],
        )?;
        Ok(())
    }

    fn update_description(&self, id: &str, description: &str) -> Result<(), StoreError> {
        self.conn.execute(
            "UPDATE experiences SET description = ?1 WHERE id = ?2",
            params![description, id],
        )?;
        Ok(())
    }

    fn update_sensation(&self, id: &str, sensation: &str) -> Result<(), StoreError> {
        self.conn.execute(
            "UPDATE experiences SET sensation = ?1 WHERE id = ?2",
            params![sensation, id],
        )?;
        Ok(())
    }
}
