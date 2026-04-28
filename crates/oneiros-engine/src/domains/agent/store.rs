use rusqlite::params;

use crate::*;

/// Agent projection store — projection lifecycle, write operations, and sync read queries.
pub struct AgentStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> AgentStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        if let Event::Known(Events::Agent(agent_event)) = &event.data {
            match agent_event {
                AgentEvents::AgentCreated(agent_created) => self.create(agent_created)?,
                AgentEvents::AgentUpdated(agent_updated) => self.update(agent_updated)?,
                AgentEvents::AgentRemoved(agent_removed) => self.remove(agent_removed)?,
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
            "create table if not exists agents (
                id          text primary key,
                name        text not null unique,
                persona     text not null default '',
                description text not null default '',
                prompt      text not null default ''
            )",
        )?;
        Ok(())
    }

    pub fn get(&self, name: &AgentName) -> Result<Option<Agent>, EventError> {
        let mut stmt = self
            .conn
            .prepare("select id, name, persona, description, prompt from agents where name = ?1")?;

        let result = stmt.query_row(params![name.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
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

    /// Look up an agent's name by its ID.
    ///
    /// Used when only an `AgentId` is available (e.g. from a foreign-key reference
    /// in an event payload) and we need to resolve it to an `AgentName`.
    pub fn get_name_by_id(&self, id: &AgentId) -> Result<Option<AgentName>, EventError> {
        let result = self.conn.query_row(
            "SELECT name FROM agents WHERE id = ?1",
            params![id.to_string()],
            |row| row.get::<_, String>(0),
        );

        match result {
            Ok(name) => Ok(Some(AgentName::new(name))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn name_exists(&self, name: &AgentName) -> Result<bool, EventError> {
        let count: i64 = self.conn.query_row(
            "select count(*) from agents where name = ?1",
            params![name.to_string()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    fn create(&self, creation: &AgentCreated) -> Result<(), EventError> {
        let agent = creation.current()?.agent;
        self.write_agent(&agent)
    }

    fn update(&self, update: &AgentUpdated) -> Result<(), EventError> {
        let agent = update.current()?.agent;
        self.write_agent(&agent)
    }

    fn remove(&self, removal: &AgentRemoved) -> Result<(), EventError> {
        let name = removal.current()?.name;
        self.conn.execute(
            "delete from agents where name = ?1",
            params![name.to_string()],
        )?;
        Ok(())
    }

    fn write_agent(&self, agent: &Agent) -> Result<(), EventError> {
        self.conn.execute(
            "insert or replace into agents (id, name, persona, description, prompt)
             values (?1, ?2, ?3, ?4, ?5)",
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
}
