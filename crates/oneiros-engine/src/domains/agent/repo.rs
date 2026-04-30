use std::collections::HashMap;

use rusqlite::{params, params_from_iter};

use crate::*;

/// Agent read model — async queries over the projection read model.
pub struct AgentRepo<'a> {
    scope: &'a Scope<AtBookmark>,
}

impl<'a> AgentRepo<'a> {
    pub fn new(scope: &'a Scope<AtBookmark>) -> Self {
        Self { scope }
    }

    pub async fn get(&self, name: &AgentName) -> Result<Option<Agent>, EventError> {
        let db = self.scope.bookmark_db()?;
        let mut stmt = db
            .prepare("select id, name, persona, description, prompt from agents where name = ?1")?;

        let result = stmt.query_row(params![name.to_string()], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            Ok((
                id,
                name,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        });

        match result {
            Ok((id, name, persona, description, prompt)) => Ok(Some(
                Agent::builder()
                    .id(id.parse()?)
                    .name(name)
                    .persona(persona)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn get_by_id(&self, id: AgentId) -> Result<Option<Agent>, EventError> {
        let db = self.scope.bookmark_db()?;
        let mut stmt =
            db.prepare("select id, name, persona, description, prompt from agents where id = ?1")?;

        let result = stmt.query_row(params![id.to_string()], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            Ok((
                id,
                name,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        });

        match result {
            Ok((id, name, persona, description, prompt)) => Ok(Some(
                Agent::builder()
                    .id(id.parse()?)
                    .name(name)
                    .persona(persona)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Hydrate many agents by id, preserving the input order. Used by list
    /// endpoints to bulk-fetch search hits in a single round trip.
    pub async fn get_many(&self, ids: &[AgentId]) -> Result<Vec<Agent>, EventError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let db = self.scope.bookmark_db()?;
        let placeholders = (1..=ids.len())
            .map(|i| format!("?{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "select id, name, persona, description, prompt
             from agents where id in ({placeholders})"
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

        let mut by_id: HashMap<AgentId, Agent> = HashMap::with_capacity(rows.len());
        for (id, name, persona, description, prompt) in rows {
            let agent = Agent::builder()
                .id(id.parse()?)
                .name(name)
                .persona(persona)
                .description(description)
                .prompt(prompt)
                .build();
            by_id.insert(agent.id, agent);
        }
        Ok(ids.iter().filter_map(|id| by_id.remove(id)).collect())
    }

    pub async fn name_exists(&self, name: &AgentName) -> Result<bool, EventError> {
        let db = self.scope.bookmark_db()?;
        let count: i64 = db.query_row(
            "select count(*) from agents where name = ?1",
            params![name.to_string()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }
}
