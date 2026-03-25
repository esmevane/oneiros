use rusqlite::params;

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

    pub async fn list(&self, agent: Option<&str>) -> Result<Vec<Memory>, EventError> {
        let db = self.context.db()?;
        let mut stmt = match agent {
            Some(_) => db.prepare(
                "SELECT id, agent_id, level, content, created_at
                 FROM memories WHERE agent_id = ?1 ORDER BY created_at",
            )?,
            None => db.prepare(
                "SELECT id, agent_id, level, content, created_at
                 FROM memories ORDER BY created_at",
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

        let mut memories = vec![];
        for (id, agent_id, level, content, created_at) in raw {
            memories.push(
                Memory::builder()
                    .id(id.parse()?)
                    .agent_id(agent_id.parse()?)
                    .level(level)
                    .content(content)
                    .created_at(Timestamp::parse_str(&created_at)?)
                    .build(),
            );
        }

        Ok(memories)
    }
}
