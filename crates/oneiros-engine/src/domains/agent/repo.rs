use rusqlite::{Connection, params};

use crate::*;

/// Agent read model — queries, projection handling, and lifecycle.
pub struct AgentRepo<'a> {
    conn: &'a Connection,
}

impl<'a> AgentRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Events::Agent(agent_event) = &event.data {
            match agent_event {
                AgentEvents::AgentCreated(agent) => self.create_record(agent)?,
                AgentEvents::AgentUpdated(agent) => self.update(agent)?,
                AgentEvents::AgentRemoved(removed) => self.remove(&removed.name)?,
            }
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM agents", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS agents (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                persona TEXT NOT NULL DEFAULT '',
                description TEXT NOT NULL DEFAULT '',
                prompt TEXT NOT NULL DEFAULT ''
            )",
        )?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn get(&self, name: &str) -> Result<Option<Agent>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, persona, description, prompt FROM agents WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
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

    pub fn list(&self) -> Result<Vec<Agent>, EventError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, persona, description, prompt FROM agents ORDER BY name")?;

        let raw: Vec<(String, String, String, String, String)> = stmt
            .query_map([], |row| {
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
            agents.push(
                Agent::builder()
                    .id(id.parse()?)
                    .name(name)
                    .persona(persona)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            );
        }

        Ok(agents)
    }

    pub fn name_exists(&self, name: &str) -> Result<bool, EventError> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM agents WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, agent: &Agent) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO agents (id, name, persona, description, prompt)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                agent.id.to_string(),
                agent.name.to_string(),
                agent.persona.to_string(),
                agent.description.to_string(),
                agent.prompt.to_string()
            ],
        )?;
        Ok(())
    }

    fn update(&self, agent: &Agent) -> Result<(), EventError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO agents (id, name, persona, description, prompt)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                agent.id.to_string(),
                agent.name.to_string(),
                agent.persona.to_string(),
                agent.description.to_string(),
                agent.prompt.to_string()
            ],
        )?;
        Ok(())
    }

    fn remove(&self, name: &AgentName) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM agents WHERE name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }
}
