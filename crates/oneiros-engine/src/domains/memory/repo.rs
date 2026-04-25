use std::collections::HashMap;

use rusqlite::{params, params_from_iter};

use crate::*;

/// Memory read model — async queries over the projection read model.
pub struct MemoryRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> MemoryRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    // ── Read queries ────────────────────────────────────────────

    pub async fn get(&self, id: &MemoryId) -> Result<Option<Memory>, EventError> {
        let db = self.context.db()?;
        let mut stmt = db.prepare(
            "SELECT id, agent_id, level, content, created_at
             FROM memories WHERE id = ?1",
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
            Ok((id, agent_id, level, content, created_at)) => Ok(Some(
                Memory::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .level(level)
                    .content(content)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Hydrate many memories by id, preserving the input order. Used by
    /// list endpoints to bulk-fetch search hits in a single round trip.
    pub async fn get_many(&self, ids: &[MemoryId]) -> Result<Vec<Memory>, EventError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let db = self.context.db()?;
        let placeholders = (1..=ids.len())
            .map(|i| format!("?{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "SELECT id, agent_id, level, content, created_at
             FROM memories WHERE id IN ({placeholders})"
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

        let mut by_id: HashMap<MemoryId, Memory> = HashMap::with_capacity(rows.len());
        for (id, agent_id, level, content, created_at) in rows {
            let memory = Memory::builder()
                .id(id.parse()?)
                .agent_id(agent_id.parse()?)
                .level(level)
                .content(content)
                .created_at(Timestamp::parse_str(&created_at)?)
                .build();
            by_id.insert(memory.id, memory);
        }
        Ok(ids.iter().filter_map(|id| by_id.remove(id)).collect())
    }
}
