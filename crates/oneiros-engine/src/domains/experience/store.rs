use rusqlite::params;

use crate::*;

/// Experience projection store — projection lifecycle, write operations, and sync read queries.
pub(crate) struct ExperienceStore<'a> {
    conn: &'a DbHandle<'a>,
}

impl<'a> ExperienceStore<'a> {
    pub(crate) fn new(conn: &'a DbHandle<'a>) -> Self {
        Self { conn }
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Experience(experience_event)) = &event.data {
            match experience_event {
                ExperienceEvents::ExperienceCreated(created) => {
                    let experience = created.current()?.experience;
                    self.write_experience(&experience)?;
                    SearchStore::new(self.conn)
                        .index_entry(&IndexEntry::experience(&experience))?;
                }
                ExperienceEvents::ExperienceDescriptionUpdated(updated) => {
                    let current = updated.current()?;
                    self.update_description(&current.id, &current.description)?;
                    if let Some(exp) = self.get(&current.id)? {
                        let search = SearchStore::new(self.conn);
                        search.remove_by_ref(&Ref::experience(exp.id))?;
                        search.index_entry(&IndexEntry::experience(&exp))?;
                    }
                }
                ExperienceEvents::ExperienceSensationUpdated(updated) => {
                    let current = updated.current()?;
                    self.update_sensation(&current.id, &current.sensation)?;
                    if let Some(exp) = self.get(&current.id)? {
                        let search = SearchStore::new(self.conn);
                        search.remove_by_ref(&Ref::experience(exp.id))?;
                        search.index_entry(&IndexEntry::experience(&exp))?;
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM experiences", [])?;
        Ok(())
    }

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
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

    pub(crate) fn get(&self, id: &ExperienceId) -> Result<Option<Experience>, EventError> {
        let result = self.conn.query_row(
            "SELECT id, agent_id, sensation, description, created_at
             FROM experiences WHERE id = ?1",
            params![id.to_string()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok((id, agent_id, sensation, description, created_at)) => Ok(Some(
                Experience::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .sensation(sensation)
                    .description(description)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            )),
            Err(DbError::Rusqlite(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) fn list(&self, agent: Option<&str>) -> Result<Vec<Experience>, EventError> {
        let sql = match agent {
            Some(_) => {
                "SELECT id, agent_id, sensation, description, created_at
                 FROM experiences WHERE agent_id = ?1 ORDER BY created_at"
            }
            None => {
                "SELECT id, agent_id, sensation, description, created_at
                 FROM experiences ORDER BY created_at"
            }
        };

        let map_row = |row: &DbRow<'_>| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        };

        let raw = match agent {
            Some(a) => self.conn.query_map(sql, params![a], map_row),
            None => self.conn.query_map(sql, [], map_row),
        }?;

        let mut experiences = vec![];
        for (id, agent_id, sensation, description, created_at) in raw {
            experiences.push(
                Experience::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .sensation(sensation)
                    .description(description)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            );
        }

        Ok(experiences)
    }

    /// Most recent experiences for an agent, ordered newest-first.
    pub(crate) fn list_recent(
        &self,
        agent_id: &str,
        limit: usize,
    ) -> Result<Vec<Experience>, EventError> {
        let raw = self.conn.query_map(
            "SELECT id, agent_id, sensation, description, created_at
             FROM experiences
             WHERE agent_id = ?1
             ORDER BY created_at DESC
             LIMIT ?2",
            params![agent_id, limit],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        )?;

        let mut experiences = vec![];
        for (id, agent_id, sensation, description, created_at) in raw {
            experiences.push(
                Experience::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .sensation(sensation)
                    .description(description)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            );
        }

        Ok(experiences)
    }

    fn write_experience(&self, experience: &Experience) -> Result<(), EventError> {
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

    fn update_description(
        &self,
        id: &ExperienceId,
        description: &Description,
    ) -> Result<(), EventError> {
        self.conn.execute(
            "UPDATE experiences SET description = ?1 WHERE id = ?2",
            params![description.to_string(), id.to_string()],
        )?;
        Ok(())
    }

    fn update_sensation(
        &self,
        id: &ExperienceId,
        sensation: &SensationName,
    ) -> Result<(), EventError> {
        self.conn.execute(
            "UPDATE experiences SET sensation = ?1 WHERE id = ?2",
            params![sensation.to_string(), id.to_string()],
        )?;
        Ok(())
    }
}
