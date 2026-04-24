use rusqlite::params;

use crate::*;

/// Cognition read model — async queries over the projection read model.
pub struct CognitionRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> CognitionRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    // ── Read queries ────────────────────────────────────────────

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
