use rusqlite::params;

use crate::*;

/// Agent read model — async queries over the projection read model.
pub struct AgentRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> AgentRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    pub async fn get(&self, name: &AgentName) -> Result<Option<Agent>, EventError> {
        let db = self.context.db()?;
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
            Ok((id, name, persona, description, prompt)) => Ok(Some(Agent::Current(
                Agent::build_v1()
                    .id(id.parse()?)
                    .name(name)
                    .persona(persona)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            ))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn get_by_id(&self, id: AgentId) -> Result<Option<Agent>, EventError> {
        let db = self.context.db()?;
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
            Ok((id, name, persona, description, prompt)) => Ok(Some(Agent::Current(
                Agent::build_v1()
                    .id(id.parse()?)
                    .name(name)
                    .persona(persona)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            ))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list(&self, filters: &SearchFilters) -> Result<Listed<Agent>, EventError> {
        let db = self.context.db()?;

        let total: usize = db.query_row("SELECT COUNT(*) FROM agents", [], |row| row.get(0))?;

        let mut stmt = db.prepare(
            "SELECT id, name, persona, description, prompt
             FROM agents ORDER BY name
             LIMIT ?1 OFFSET ?2",
        )?;

        let raw: Vec<(String, String, String, String, String)> = stmt
            .query_map(params![filters.limit, filters.offset], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut agents = vec![];

        for (id, name, persona, description, prompt) in raw {
            agents.push(Agent::Current(
                Agent::build_v1()
                    .id(id.parse()?)
                    .name(name)
                    .persona(persona)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            ));
        }

        Ok(Listed::new(agents, total))
    }

    pub async fn name_exists(&self, name: &AgentName) -> Result<bool, EventError> {
        let db = self.context.db()?;
        let count: i64 = db.query_row(
            "select count(*) from agents where name = ?1",
            params![name.to_string()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }
}
