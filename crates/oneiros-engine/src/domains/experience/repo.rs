use std::collections::HashMap;

use rusqlite::{params, params_from_iter};

use crate::*;

/// Experience read model — async queries over the projection read model.
pub struct ExperienceRepo<'a> {
    scope: &'a Scope<AtBookmark>,
}

impl<'a> ExperienceRepo<'a> {
    pub fn new(scope: &'a Scope<AtBookmark>) -> Self {
        Self { scope }
    }

    pub async fn get(&self, id: &ExperienceId) -> Result<Option<Experience>, EventError> {
        let db = self.scope.bookmark_db().await?;
        let mut stmt = db.prepare(
            "SELECT id, agent_id, sensation, description, created_at
             FROM experiences WHERE id = ?1",
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
            Ok((id, agent_id, sensation, description, created_at)) => Ok(Some(
                Experience::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .sensation(sensation)
                    .description(description)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Hydrate many experiences by id, preserving the input order. Used by
    /// list endpoints to bulk-fetch search hits in a single round trip.
    pub async fn get_many(&self, ids: &[ExperienceId]) -> Result<Vec<Experience>, EventError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let db = self.scope.bookmark_db().await?;
        let placeholders = (1..=ids.len())
            .map(|i| format!("?{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "SELECT id, agent_id, sensation, description, created_at
             FROM experiences WHERE id IN ({placeholders})"
        );
        let id_strs: Vec<String> = ids.iter().map(ToString::to_string).collect();
        let mut stmt = db.prepare(&sql)?;
        let rows = stmt
            .query_map(params_from_iter(id_strs.iter()), |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut by_id: HashMap<ExperienceId, Experience> = HashMap::with_capacity(rows.len());
        for (id, agent_id, sensation, description, created_at) in rows {
            let experience = Experience::builder()
                .id(id.parse()?)
                .agent_id(agent_id.parse()?)
                .sensation(sensation)
                .description(description)
                .created_at(Timestamp::parse_str(&created_at)?)
                .build();
            by_id.insert(experience.id, experience);
        }
        Ok(ids.iter().filter_map(|id| by_id.remove(id)).collect())
    }

    /// Most recent experiences for an agent, ordered newest-first.
    pub async fn list_recent(
        &self,
        agent_id: &str,
        limit: usize,
    ) -> Result<Vec<Experience>, EventError> {
        let db = self.scope.bookmark_db().await?;
        let mut stmt = db.prepare(
            "SELECT id, agent_id, sensation, description, created_at
             FROM experiences
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
}
