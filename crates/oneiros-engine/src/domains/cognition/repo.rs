use rusqlite::{Connection, params};

use crate::events::Events;
use crate::store::{StoreError, StoredEvent};

use super::events::*;
use super::model::Cognition;

/// Cognition read model — queries, projection handling, and lifecycle.
pub struct CognitionRepo<'a> {
    conn: &'a Connection,
}

impl<'a> CognitionRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        if let Events::Cognition(cognition_event) = &event.data {
            match cognition_event {
                CognitionEvents::CognitionAdded(cognition) => self.insert(cognition)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM cognitions", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cognitions (
                id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                texture TEXT NOT NULL DEFAULT '',
                content TEXT NOT NULL DEFAULT '',
                created_at TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, id: &str) -> Result<Option<Cognition>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, texture, content, created_at
             FROM cognitions WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id], |row| {
            Ok(Cognition {
                id: row.get(0)?,
                agent_id: row.get(1)?,
                texture: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        });

        match result {
            Ok(c) => Ok(Some(c)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(
        &self,
        agent: Option<&str>,
        texture: Option<&str>,
    ) -> Result<Vec<Cognition>, StoreError> {
        // Build query dynamically based on filters present.
        let sql = match (agent, texture) {
            (Some(_), Some(_)) => {
                "SELECT id, agent_id, texture, content, created_at
                 FROM cognitions
                 WHERE agent_id = ?1 AND texture = ?2
                 ORDER BY created_at"
            }
            (Some(_), None) => {
                "SELECT id, agent_id, texture, content, created_at
                 FROM cognitions
                 WHERE agent_id = ?1
                 ORDER BY created_at"
            }
            (None, Some(_)) => {
                "SELECT id, agent_id, texture, content, created_at
                 FROM cognitions
                 WHERE texture = ?1
                 ORDER BY created_at"
            }
            (None, None) => {
                "SELECT id, agent_id, texture, content, created_at
                 FROM cognitions
                 ORDER BY created_at"
            }
        };

        let mut stmt = self.conn.prepare(sql)?;

        let map_row = |row: &rusqlite::Row<'_>| {
            Ok(Cognition {
                id: row.get(0)?,
                agent_id: row.get(1)?,
                texture: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        };

        let cognitions = match (agent, texture) {
            (Some(a), Some(t)) => stmt.query_map(params![a, t], map_row),
            (Some(a), None) => stmt.query_map(params![a], map_row),
            (None, Some(t)) => stmt.query_map(params![t], map_row),
            (None, None) => stmt.query_map([], map_row),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(cognitions)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn insert(&self, cognition: &Cognition) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO cognitions (id, agent_id, texture, content, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                cognition.id,
                cognition.agent_id,
                cognition.texture,
                cognition.content,
                cognition.created_at,
            ],
        )?;
        Ok(())
    }
}
