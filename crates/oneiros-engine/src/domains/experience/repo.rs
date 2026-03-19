use rusqlite::{Connection, params};

use crate::*;

/// Experience read model — queries, projection handling, and lifecycle.
pub struct ExperienceRepo<'a> {
    conn: &'a Connection,
}

impl<'a> ExperienceRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Experience(experience_event) = &event.data {
            match experience_event {
                ExperienceEvents::ExperienceCreated(experience) => self.insert(experience)?,
                ExperienceEvents::ExperienceDescriptionUpdated(update) => {
                    self.update_description(&update.id.to_string(), &update.description.to_string())?
                }
                ExperienceEvents::ExperienceSensationUpdated(update) => {
                    self.update_sensation(&update.id.to_string(), &update.sensation.to_string())?
                }
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM experiences", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
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

    pub fn get(&self, id: &str) -> Result<Option<Experience>, EventError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, agent_id, sensation, description, created_at
             FROM experiences WHERE id = ?1",
        )?;

        let result = stmt.query_row(params![id], |row| {
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
            Ok((id, agent_id, sensation, description, created_at)) => Ok(Some(Experience {
                id: id.parse()?,
                agent_id: AgentName::new(agent_id),
                sensation: SensationName::new(sensation),
                description: Description(description),
                created_at: Timestamp::parse_str(&created_at)?,
            })),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self, agent: Option<&str>) -> Result<Vec<Experience>, EventError> {
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
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        };

        let raw = match agent {
            Some(a) => stmt.query_map(params![a], map_row),
            None => stmt.query_map([], map_row),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        let mut experiences = vec![];
        for (id, agent_id, sensation, description, created_at) in raw {
            experiences.push(Experience {
                id: id.parse()?,
                agent_id: AgentName::new(agent_id),
                sensation: SensationName::new(sensation),
                description: Description(description),
                created_at: Timestamp::parse_str(&created_at)?,
            });
        }

        Ok(experiences)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn insert(&self, experience: &Experience) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO experiences (id, agent_id, sensation, description, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                experience.id.to_string(),
                experience.agent_id.to_string(),
                experience.sensation.to_string(),
                experience.description.to_string(),
                experience.created_at.as_string(),
            ],
        )?;
        Ok(())
    }

    fn update_description(&self, id: &str, description: &str) -> Result<(), EventError> {
        self.conn.execute(
            "UPDATE experiences SET description = ?1 WHERE id = ?2",
            params![description, id],
        )?;
        Ok(())
    }

    fn update_sensation(&self, id: &str, sensation: &str) -> Result<(), EventError> {
        self.conn.execute(
            "UPDATE experiences SET sensation = ?1 WHERE id = ?2",
            params![sensation, id],
        )?;
        Ok(())
    }
}
