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

    // ── Read queries ────────────────────────────────────────────

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
