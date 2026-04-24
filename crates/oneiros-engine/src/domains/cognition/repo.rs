use std::collections::HashMap;

use rusqlite::{params, params_from_iter};

use crate::*;

/// Cognition read model — async queries over the projection read model.
pub struct CognitionRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> CognitionRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    pub async fn get(&self, id: &CognitionId) -> Result<Option<Cognition>, EventError> {
        let db = self.context.db()?;
        let mut stmt = db.prepare(
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

    /// Hydrate many cognitions by id, preserving the input order. Used by
    /// list endpoints to bulk-fetch search hits in a single round trip.
    pub async fn get_many(&self, ids: &[CognitionId]) -> Result<Vec<Cognition>, EventError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let db = self.context.db()?;
        let placeholders = (1..=ids.len())
            .map(|i| format!("?{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "SELECT id, agent_id, texture, content, created_at
             FROM cognitions WHERE id IN ({placeholders})"
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

        let mut by_id: HashMap<CognitionId, Cognition> = HashMap::with_capacity(rows.len());
        for (id, agent_id, texture, content, created_at) in rows {
            let cognition = Cognition::builder()
                .id(id.parse()?)
                .agent_id(agent_id.parse()?)
                .texture(texture)
                .content(content)
                .created_at(Timestamp::parse_str(&created_at)?)
                .build();
            by_id.insert(cognition.id, cognition);
        }
        Ok(ids.iter().filter_map(|id| by_id.remove(id)).collect())
    }

    /// Most recent cognitions for an agent, ordered newest-first.
    pub async fn list_recent(
        &self,
        agent_id: &AgentId,
        limit: usize,
    ) -> Result<Vec<Cognition>, EventError> {
        let db = self.context.db()?;
        let mut stmt = db.prepare(
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
            .query_map(params![agent_id.to_string(), limit], map_row)?
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
}
