use rusqlite::{Connection, params};

use crate::*;

/// Cognition read model — queries, projection handling, and lifecycle.
pub struct CognitionRepo<'a> {
    conn: &'a Connection,
}

impl<'a> CognitionRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Cognition(cognition_event) = &event.data {
            match cognition_event {
                CognitionEvents::CognitionAdded(cognition) => self.insert(cognition)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM cognitions", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
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

    pub fn get(&self, id: &CognitionId) -> Result<Option<Cognition>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, texture, content, created_at
             FROM cognitions WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id.to_string()], |row| {
            let id: String = row.get(0)?;
            Ok((
                id,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        });

        match result {
            Ok((id, agent_id, texture, content, created_at)) => Ok(Some(
                Cognition::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .texture(texture)
                    .content(content)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(
        &self,
        agent: Option<&str>,
        texture: Option<&str>,
    ) -> Result<Vec<Cognition>, EventError> {
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
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        };

        let raw = match (agent, texture) {
            (Some(a), Some(t)) => stmt.query_map(params![a, t], map_row),
            (Some(a), None) => stmt.query_map(params![a], map_row),
            (None, Some(t)) => stmt.query_map(params![t], map_row),
            (None, None) => stmt.query_map([], map_row),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        let mut cognitions = vec![];
        for (id, agent_id, texture, content, created_at) in raw {
            cognitions.push(
                Cognition::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .texture(texture)
                    .content(content)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            );
        }

        Ok(cognitions)
    }

    /// Most recent cognitions for an agent, ordered newest-first.
    pub fn list_recent(&self, agent_id: &str, limit: usize) -> Result<Vec<Cognition>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, texture, content, created_at
             FROM cognitions
             WHERE agent_id = ?1
             ORDER BY created_at DESC
             LIMIT ?2",
        )?;

        let map_row = |row: &rusqlite::Row<'_>| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        };

        let raw = stmt
            .query_map(params![agent_id, limit], map_row)?
            .collect::<Result<Vec<_>, _>>()?;

        let mut cognitions = vec![];
        for (id, agent_id, texture, content, created_at) in raw {
            cognitions.push(
                Cognition::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .texture(texture)
                    .content(content)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            );
        }

        Ok(cognitions)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn insert(&self, cognition: &Cognition) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO cognitions (id, agent_id, texture, content, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                cognition.id.to_string(),
                cognition.agent_id.to_string(),
                cognition.texture.to_string(),
                cognition.content.to_string(),
                cognition.created_at.as_string(),
            ],
        )?;
        Ok(())
    }
}
