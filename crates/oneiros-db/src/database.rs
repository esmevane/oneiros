use rusqlite::functions::FunctionFlags;
use rusqlite::{Connection, params};
use serde_json::Value;
use std::path::Path;
use uuid::Uuid;

use crate::*;

/// Raw row from the agent table: (id, name, persona, description, prompt).
type AgentRow = (String, String, String, String, String);

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Register application-defined functions on a connection.
    fn register_functions(conn: &Connection) -> Result<(), DatabaseError> {
        conn.create_scalar_function(
            "uuid",
            0,
            FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
            |_| Ok(Uuid::new_v4().to_string()),
        )?;
        Ok(())
    }

    pub fn open(connection_string: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let conn = Connection::open(connection_string.as_ref())?;
        Self::register_functions(&conn)?;

        Ok(Self { conn })
    }

    pub fn create(path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let conn = Connection::open(path)?;
        Self::register_functions(&conn)?;

        conn.execute_batch(migrations::SYSTEM)?;

        Ok(Self { conn })
    }

    pub fn event_count(&self) -> Result<usize, DatabaseError> {
        let count: i64 = self
            .conn
            .query_row("select count(*) from events", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    pub fn tenant_exists(&self) -> Result<bool, DatabaseError> {
        let count: i64 = self
            .conn
            .query_row("select count(*) from tenant", [], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn reset_tenants(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from tenant")?;
        Ok(())
    }

    pub fn reset_actors(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from actor")?;
        Ok(())
    }

    pub fn create_tenant(
        &self,
        tenant_id: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into tenant (id, name) values (?1, ?2)",
            params![tenant_id.as_ref(), name.as_ref()],
        )?;
        Ok(())
    }

    pub fn create_actor(
        &self,
        actor_id: impl AsRef<str>,
        tenant_id: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into actor (id, tenant_id, name) values (?1, ?2, ?3)",
            params![actor_id.as_ref(), tenant_id.as_ref(), name.as_ref()],
        )?;
        Ok(())
    }

    pub fn get_tenant_id(&self) -> Result<Option<String>, DatabaseError> {
        let result = self
            .conn
            .query_row("select id from tenant limit 1", [], |row| row.get(0));

        match result {
            Ok(id) => Ok(Some(id)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn get_brain_path(
        &self,
        tenant_id: impl AsRef<str>,
        id: impl AsRef<str>,
    ) -> Result<Option<String>, DatabaseError> {
        let result = self.conn.query_row(
            "select path from brain where tenant_id = ?1 and id = ?2",
            params![tenant_id.as_ref(), id.as_ref()],
            |row| row.get(0),
        );

        match result {
            Ok(path) => Ok(Some(path)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn brain_exists(
        &self,
        tenant_id: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<bool, DatabaseError> {
        let count: i64 = self.conn.query_row(
            "select count(*) from brain where tenant_id = ?1 and name = ?2",
            params![tenant_id.as_ref(), name.as_ref()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn create_brain(
        &self,
        brain_id: impl AsRef<str>,
        tenant_id: impl AsRef<str>,
        name: impl AsRef<str>,
        path: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into brain (id, tenant_id, name, path) values (?1, ?2, ?3, ?4)",
            params![
                brain_id.as_ref(),
                tenant_id.as_ref(),
                name.as_ref(),
                path.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn reset_brains(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from brain")?;
        Ok(())
    }

    pub fn get_actor_id(
        &self,
        tenant_id: impl AsRef<str>,
    ) -> Result<Option<String>, DatabaseError> {
        let result = self.conn.query_row(
            "select id from actor where tenant_id = ?1 limit 1",
            params![tenant_id.as_ref()],
            |row| row.get(0),
        );

        match result {
            Ok(id) => Ok(Some(id)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn create_ticket(
        &self,
        ticket_id: impl AsRef<str>,
        token: impl AsRef<str>,
        created_by: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into tickets (id, token, created_by) values (?1, ?2, ?3)",
            params![ticket_id.as_ref(), token.as_ref(), created_by.as_ref()],
        )?;
        Ok(())
    }

    pub fn validate_ticket(&self, token: impl AsRef<str>) -> Result<bool, DatabaseError> {
        let result = self.conn.query_row(
            "select id from tickets \
             where token = ?1 \
             and revoked_on is null \
             and (expires_at is null or expires_at > strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) \
             and (max_uses is null or uses < max_uses)",
            params![token.as_ref()],
            |row| row.get::<_, String>(0),
        );

        match result {
            Ok(_) => Ok(true),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
            Err(error) => Err(error.into()),
        }
    }

    pub fn reset_tickets(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from tickets")?;
        Ok(())
    }

    pub fn set_persona(
        &self,
        name: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into persona (name, description, prompt) \
             values (?1, ?2, ?3) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt",
            params![name.as_ref(), description.as_ref(), prompt.as_ref()],
        )?;
        Ok(())
    }

    pub fn remove_persona(&self, name: impl AsRef<str>) -> Result<(), DatabaseError> {
        self.conn.execute(
            "delete from persona where name = ?1",
            params![name.as_ref()],
        )?;
        Ok(())
    }

    pub fn get_persona(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Option<(String, String, String)>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from persona where name = ?1",
            params![name.as_ref()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        );

        match result {
            Ok(persona) => Ok(Some(persona)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_personas(&self) -> Result<Vec<(String, String, String)>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select name, description, prompt from persona order by name")?;

        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

        let mut personas = Vec::new();
        for row in rows {
            personas.push(row?);
        }
        Ok(personas)
    }

    pub fn reset_personas(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from persona")?;
        Ok(())
    }

    pub fn set_texture(
        &self,
        name: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into texture (name, description, prompt) \
             values (?1, ?2, ?3) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt",
            params![name.as_ref(), description.as_ref(), prompt.as_ref()],
        )?;
        Ok(())
    }

    pub fn remove_texture(&self, name: impl AsRef<str>) -> Result<(), DatabaseError> {
        self.conn.execute(
            "delete from texture where name = ?1",
            params![name.as_ref()],
        )?;
        Ok(())
    }

    pub fn get_texture(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Option<(String, String, String)>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from texture where name = ?1",
            params![name.as_ref()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        );

        match result {
            Ok(texture) => Ok(Some(texture)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_textures(&self) -> Result<Vec<(String, String, String)>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select name, description, prompt from texture order by name")?;

        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

        let mut textures = Vec::new();
        for row in rows {
            textures.push(row?);
        }
        Ok(textures)
    }

    pub fn reset_textures(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from texture")?;
        Ok(())
    }

    pub fn set_level(
        &self,
        name: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into level (name, description, prompt) \
             values (?1, ?2, ?3) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt",
            params![name.as_ref(), description.as_ref(), prompt.as_ref()],
        )?;
        Ok(())
    }

    pub fn remove_level(&self, name: impl AsRef<str>) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from level where name = ?1", params![name.as_ref()])?;
        Ok(())
    }

    pub fn get_level(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Option<(String, String, String)>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from level where name = ?1",
            params![name.as_ref()],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        );

        match result {
            Ok(level) => Ok(Some(level)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_levels(&self) -> Result<Vec<(String, String, String)>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select name, description, prompt from level order by name")?;

        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?;

        let mut levels = Vec::new();
        for row in rows {
            levels.push(row?);
        }
        Ok(levels)
    }

    pub fn reset_levels(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from level")?;
        Ok(())
    }

    pub fn create_agent_record(
        &self,
        id: impl AsRef<str>,
        name: impl AsRef<str>,
        persona: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into agent (id, name, persona, description, prompt) \
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                id.as_ref(),
                name.as_ref(),
                persona.as_ref(),
                description.as_ref(),
                prompt.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn update_agent(
        &self,
        name: impl AsRef<str>,
        persona: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "update agent set persona = ?2, description = ?3, prompt = ?4 where name = ?1",
            params![
                name.as_ref(),
                persona.as_ref(),
                description.as_ref(),
                prompt.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn remove_agent(&self, name: impl AsRef<str>) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from agent where name = ?1", params![name.as_ref()])?;
        Ok(())
    }

    pub fn get_agent(&self, name: impl AsRef<str>) -> Result<Option<AgentRow>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, name, persona, description, prompt from agent where name = ?1",
            params![name.as_ref()],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            },
        );

        match result {
            Ok(agent) => Ok(Some(agent)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_agents(&self) -> Result<Vec<AgentRow>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select id, name, persona, description, prompt from agent order by name")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })?;

        let mut agents = Vec::new();
        for row in rows {
            agents.push(row?);
        }
        Ok(agents)
    }

    pub fn agent_name_exists(&self, name: impl AsRef<str>) -> Result<bool, DatabaseError> {
        let count: i64 = self.conn.query_row(
            "select count(*) from agent where name = ?1",
            params![name.as_ref()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn reset_agents(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from agent")?;
        Ok(())
    }

    pub fn create_brain_db(path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let conn = Connection::open(path)?;
        Self::register_functions(&conn)?;

        conn.execute_batch(migrations::BRAIN)?;

        Ok(Self { conn })
    }

    pub fn log_event(
        &self,
        data: impl serde::Serialize,
        projections: &[Projection],
    ) -> Result<(), DatabaseError> {
        let event = serde_json::to_value(data)?;

        self.create_event(&event)?;
        self.run_projections(projections, &event)
    }

    fn create_event(&self, data: &Value) -> Result<(), DatabaseError> {
        let event_type = data["type"].as_str().unwrap_or("__unmarked");
        let meta = serde_json::json!({ "type": event_type });

        self.conn.execute(
            "insert into events (data, meta) values (?1, ?2)",
            params![data.to_string(), meta.to_string()],
        )?;

        Ok(())
    }

    fn run_projections(
        &self,
        projections: &[Projection],
        event: &Value,
    ) -> Result<(), DatabaseError> {
        let Some(event_type) = event["type"].as_str() else {
            return Ok(());
        };

        let data = event["data"].clone();

        projections::project(self, projections, event_type, &data)
    }
}
