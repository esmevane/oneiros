use oneiros_model::*;
use rusqlite::functions::FunctionFlags;
use rusqlite::{Connection, params};
use serde_json::Value;
use std::path::Path;
use uuid::Uuid;

use crate::*;

/// A raw event row from the `events` table.
///
/// Used by the replay tool to read the full event log in chronological order.
pub struct EventRow {
    pub id: String,
    pub timestamp: String,
    pub data: serde_json::Value,
}

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

    /// Open a brain database, applying any missing schema migrations.
    ///
    /// The brain migration SQL is fully idempotent (`CREATE TABLE IF NOT EXISTS`),
    /// so this is safe to call on every access. It ensures existing brain databases
    /// pick up new tables introduced in later versions.
    pub fn open_brain(path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        let conn = Connection::open(path)?;
        Self::register_functions(&conn)?;
        conn.execute_batch(migrations::BRAIN)?;

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

    /// Read all events in chronological order.
    ///
    /// Events are ordered by `id ASC` (UUIDv7 IDs sort chronologically).
    /// Each row's `data` column is parsed from its stored JSON string into
    /// a `serde_json::Value`.
    pub fn read_events(&self) -> Result<Vec<EventRow>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, timestamp, data FROM events ORDER BY id ASC")?;

        let rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let timestamp: String = row.get(1)?;
            let data_str: String = row.get(2)?;

            Ok((id, timestamp, data_str))
        })?;

        let mut events = Vec::new();

        for row in rows {
            let (id, timestamp, data_str) = row?;
            let data: serde_json::Value = serde_json::from_str(&data_str)?;

            events.push(EventRow {
                id,
                timestamp,
                data,
            });
        }

        Ok(events)
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
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into tenant (id, name, link) values (?1, ?2, ?3)",
            params![tenant_id.as_ref(), name.as_ref(), link.as_ref()],
        )?;
        Ok(())
    }

    pub fn create_actor(
        &self,
        actor_id: impl AsRef<str>,
        tenant_id: impl AsRef<str>,
        name: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into actor (id, tenant_id, name, link) values (?1, ?2, ?3, ?4)",
            params![
                actor_id.as_ref(),
                tenant_id.as_ref(),
                name.as_ref(),
                link.as_ref()
            ],
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
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into brain (id, tenant_id, name, path, link) \
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                brain_id.as_ref(),
                tenant_id.as_ref(),
                name.as_ref(),
                path.as_ref(),
                link.as_ref()
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

    // -- Persona operations --

    pub fn set_persona(
        &self,
        name: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into persona (name, description, prompt, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt, \
             link = excluded.link",
            params![
                name.as_ref(),
                description.as_ref(),
                prompt.as_ref(),
                link.as_ref()
            ],
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

    pub fn get_persona(&self, name: impl AsRef<str>) -> Result<Option<Persona>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from persona where name = ?1",
            params![name.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Persona::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_personas(&self) -> Result<Vec<Persona>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select name, description, prompt from persona order by name")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut personas = Vec::new();
        for row in rows {
            personas.push(Persona::construct_from_db(row?));
        }
        Ok(personas)
    }

    pub fn get_persona_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Option<Persona>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from persona where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Persona::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn reset_personas(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from persona")?;
        Ok(())
    }

    // -- Texture operations --

    pub fn set_texture(
        &self,
        name: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into texture (name, description, prompt, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt, \
             link = excluded.link",
            params![
                name.as_ref(),
                description.as_ref(),
                prompt.as_ref(),
                link.as_ref()
            ],
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

    pub fn get_texture(&self, name: impl AsRef<str>) -> Result<Option<Texture>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from texture where name = ?1",
            params![name.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Texture::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_textures(&self) -> Result<Vec<Texture>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select name, description, prompt from texture order by name")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut textures = Vec::new();
        for row in rows {
            textures.push(Texture::construct_from_db(row?));
        }
        Ok(textures)
    }

    pub fn get_texture_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Option<Texture>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from texture where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Texture::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn reset_textures(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from texture")?;
        Ok(())
    }

    // -- Level operations --

    pub fn set_level(
        &self,
        name: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into level (name, description, prompt, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt, \
             link = excluded.link",
            params![
                name.as_ref(),
                description.as_ref(),
                prompt.as_ref(),
                link.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn remove_level(&self, name: impl AsRef<str>) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from level where name = ?1", params![name.as_ref()])?;
        Ok(())
    }

    pub fn get_level(&self, name: impl AsRef<str>) -> Result<Option<Level>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from level where name = ?1",
            params![name.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Level::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_levels(&self) -> Result<Vec<Level>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select name, description, prompt from level order by name")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut levels = Vec::new();
        for row in rows {
            levels.push(Level::construct_from_db(row?));
        }
        Ok(levels)
    }

    pub fn get_level_by_link(&self, link: impl AsRef<str>) -> Result<Option<Level>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from level where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Level::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn reset_levels(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from level")?;
        Ok(())
    }

    // -- Agent operations --

    pub fn create_agent_record(
        &self,
        id: impl AsRef<str>,
        name: impl AsRef<str>,
        persona: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into agent (id, name, persona, description, prompt, link) \
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id.as_ref(),
                name.as_ref(),
                persona.as_ref(),
                description.as_ref(),
                prompt.as_ref(),
                link.as_ref()
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
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "update agent set persona = ?2, description = ?3, prompt = ?4, link = ?5 \
             where name = ?1",
            params![
                name.as_ref(),
                persona.as_ref(),
                description.as_ref(),
                prompt.as_ref(),
                link.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn remove_agent(&self, name: impl AsRef<str>) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from agent where name = ?1", params![name.as_ref()])?;
        Ok(())
    }

    pub fn get_agent(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Option<Identity<AgentId, Agent>>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, name, persona, description, prompt from agent where name = ?1",
            params![name.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Agent::construct_from_db(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_agents(&self) -> Result<Vec<Identity<AgentId, Agent>>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select id, name, persona, description, prompt from agent order by name")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(Agent::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn get_agent_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Option<Identity<AgentId, Agent>>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, name, persona, description, prompt from agent where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Agent::construct_from_db(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
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

    // -- Cognition operations --

    pub fn add_cognition(
        &self,
        id: impl AsRef<str>,
        agent_id: impl AsRef<str>,
        texture: impl AsRef<str>,
        content: impl AsRef<str>,
        created_at: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into cognition (id, agent_id, texture, content, created_at, link) \
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id.as_ref(),
                agent_id.as_ref(),
                texture.as_ref(),
                content.as_ref(),
                created_at.as_ref(),
                link.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn get_cognition(
        &self,
        id: impl AsRef<str>,
    ) -> Result<Option<Identity<CognitionId, Cognition>>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, agent_id, texture, content, created_at from cognition where id = ?1",
            params![id.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Cognition::construct_from_db(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_cognitions(&self) -> Result<Vec<Identity<CognitionId, Cognition>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, texture, content, created_at from cognition order by rowid",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(Cognition::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn list_cognitions_by_agent(
        &self,
        agent_id: impl AsRef<str>,
    ) -> Result<Vec<Identity<CognitionId, Cognition>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, texture, content, created_at from cognition \
             where agent_id = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![agent_id.as_ref()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(Cognition::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn list_cognitions_by_texture(
        &self,
        texture: impl AsRef<str>,
    ) -> Result<Vec<Identity<CognitionId, Cognition>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, texture, content, created_at from cognition \
             where texture = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![texture.as_ref()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(Cognition::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn list_cognitions_by_agent_and_texture(
        &self,
        agent_id: impl AsRef<str>,
        texture: impl AsRef<str>,
    ) -> Result<Vec<Identity<CognitionId, Cognition>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, texture, content, created_at from cognition \
             where agent_id = ?1 and texture = ?2 order by rowid",
        )?;

        let rows = stmt.query_map(params![agent_id.as_ref(), texture.as_ref()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(Cognition::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn get_cognition_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Option<Identity<CognitionId, Cognition>>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, agent_id, texture, content, created_at from cognition where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Cognition::construct_from_db(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn reset_cognitions(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from cognition")?;
        Ok(())
    }

    // -- Memory operations --

    pub fn add_memory(
        &self,
        id: impl AsRef<str>,
        agent_id: impl AsRef<str>,
        level: impl AsRef<str>,
        content: impl AsRef<str>,
        created_at: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into memory (id, agent_id, level, content, created_at, link) \
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id.as_ref(),
                agent_id.as_ref(),
                level.as_ref(),
                content.as_ref(),
                created_at.as_ref(),
                link.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn get_memory(
        &self,
        id: impl AsRef<str>,
    ) -> Result<Option<Identity<MemoryId, Memory>>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, agent_id, level, content, created_at from memory where id = ?1",
            params![id.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Memory::construct_from_db(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_memories(&self) -> Result<Vec<Identity<MemoryId, Memory>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, level, content, created_at from memory order by rowid",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(Memory::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn list_memories_by_agent(
        &self,
        agent_id: impl AsRef<str>,
    ) -> Result<Vec<Identity<MemoryId, Memory>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, level, content, created_at from memory \
             where agent_id = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![agent_id.as_ref()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(Memory::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn list_memories_by_level(
        &self,
        level: impl AsRef<str>,
    ) -> Result<Vec<Identity<MemoryId, Memory>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, level, content, created_at from memory \
             where level = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![level.as_ref()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(Memory::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn list_memories_by_agent_and_level(
        &self,
        agent_id: impl AsRef<str>,
        level: impl AsRef<str>,
    ) -> Result<Vec<Identity<MemoryId, Memory>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, level, content, created_at from memory \
             where agent_id = ?1 and level = ?2 order by rowid",
        )?;

        let rows = stmt.query_map(params![agent_id.as_ref(), level.as_ref()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(Memory::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn get_memory_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Option<Identity<MemoryId, Memory>>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, agent_id, level, content, created_at from memory where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Memory::construct_from_db(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn reset_memories(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from memory")?;
        Ok(())
    }

    // -- Sensation operations --

    pub fn set_sensation(
        &self,
        name: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into sensation (name, description, prompt, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt, \
             link = excluded.link",
            params![
                name.as_ref(),
                description.as_ref(),
                prompt.as_ref(),
                link.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn remove_sensation(&self, name: impl AsRef<str>) -> Result<(), DatabaseError> {
        self.conn.execute(
            "delete from sensation where name = ?1",
            params![name.as_ref()],
        )?;
        Ok(())
    }

    pub fn get_sensation(&self, name: impl AsRef<str>) -> Result<Option<Sensation>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from sensation where name = ?1",
            params![name.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Sensation::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_sensations(&self) -> Result<Vec<Sensation>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select name, description, prompt from sensation order by name")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut sensations = Vec::new();
        for row in rows {
            sensations.push(Sensation::construct_from_db(row?));
        }
        Ok(sensations)
    }

    pub fn get_sensation_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Option<Sensation>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from sensation where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Sensation::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn reset_sensations(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from sensation")?;
        Ok(())
    }

    // -- Nature operations --

    pub fn set_nature(
        &self,
        name: impl AsRef<str>,
        description: impl AsRef<str>,
        prompt: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into nature (name, description, prompt, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt, \
             link = excluded.link",
            params![
                name.as_ref(),
                description.as_ref(),
                prompt.as_ref(),
                link.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn remove_nature(&self, name: impl AsRef<str>) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from nature where name = ?1", params![name.as_ref()])?;
        Ok(())
    }

    pub fn get_nature(&self, name: impl AsRef<str>) -> Result<Option<Nature>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from nature where name = ?1",
            params![name.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Nature::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_natures(&self) -> Result<Vec<Nature>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select name, description, prompt from nature order by name")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut natures = Vec::new();
        for row in rows {
            natures.push(Nature::construct_from_db(row?));
        }
        Ok(natures)
    }

    pub fn get_nature_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Option<Nature>, DatabaseError> {
        let result = self.conn.query_row(
            "select name, description, prompt from nature where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(Nature::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn reset_natures(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from nature")?;
        Ok(())
    }

    // -- Connection operations --

    pub fn create_connection(
        &self,
        id: impl AsRef<str>,
        nature: impl AsRef<str>,
        from_link: impl AsRef<str>,
        to_link: impl AsRef<str>,
        created_at: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into connection (id, nature, from_link, to_link, created_at, link) \
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id.as_ref(),
                nature.as_ref(),
                from_link.as_ref(),
                to_link.as_ref(),
                created_at.as_ref(),
                link.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn get_connection(
        &self,
        id: impl AsRef<str>,
    ) -> Result<Option<Identity<ConnectionId, oneiros_model::Connection>>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, nature, from_link, to_link, created_at from connection where id = ?1",
            params![id.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(oneiros_model::Connection::construct_from_db(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_connections(
        &self,
    ) -> Result<Vec<Identity<ConnectionId, oneiros_model::Connection>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, nature, from_link, to_link, created_at from connection order by rowid",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(oneiros_model::Connection::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn list_connections_by_nature(
        &self,
        nature: impl AsRef<str>,
    ) -> Result<Vec<Identity<ConnectionId, oneiros_model::Connection>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, nature, from_link, to_link, created_at from connection \
             where nature = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![nature.as_ref()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(oneiros_model::Connection::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn list_connections_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Vec<Identity<ConnectionId, oneiros_model::Connection>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, nature, from_link, to_link, created_at from connection \
             where from_link = ?1 or to_link = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![link.as_ref()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        raw_rows
            .into_iter()
            .map(oneiros_model::Connection::construct_from_db)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DatabaseError::from)
    }

    pub fn get_connection_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Option<Identity<ConnectionId, oneiros_model::Connection>>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, nature, from_link, to_link, created_at from connection where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(oneiros_model::Connection::construct_from_db(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn remove_connection(&self, id: impl AsRef<str>) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from connection where id = ?1", params![id.as_ref()])?;
        Ok(())
    }

    pub fn reset_connections(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from connection")?;
        Ok(())
    }

    // -- Experience operations --

    pub fn add_experience(
        &self,
        id: impl AsRef<str>,
        agent_id: impl AsRef<str>,
        sensation: impl AsRef<str>,
        description: impl AsRef<str>,
        created_at: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into experience (id, agent_id, sensation, description, created_at, link) \
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id.as_ref(),
                agent_id.as_ref(),
                sensation.as_ref(),
                description.as_ref(),
                created_at.as_ref(),
                link.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn get_experience(
        &self,
        id: impl AsRef<str>,
    ) -> Result<Option<Identity<ExperienceId, Experience>>, DatabaseError> {
        let id_ref = id.as_ref();
        let result = self.conn.query_row(
            "select id, agent_id, sensation, description, created_at from experience where id = ?1",
            params![id_ref],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok(row) => {
                let refs = self.collect_experience_refs(id_ref)?;
                Ok(Some(Experience::construct_from_db(row, refs)?))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_experiences(
        &self,
    ) -> Result<Vec<Identity<ExperienceId, Experience>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, sensation, description, created_at from experience order by rowid",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<(String, String, String, String, String)> =
            rows.collect::<Result<_, _>>()?;

        let mut experiences = Vec::new();
        for row in raw_rows {
            let refs = self.collect_experience_refs(&row.0)?;
            experiences.push(Experience::construct_from_db(row, refs)?);
        }
        Ok(experiences)
    }

    pub fn list_experiences_by_agent(
        &self,
        agent_id: impl AsRef<str>,
    ) -> Result<Vec<Identity<ExperienceId, Experience>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, sensation, description, created_at from experience \
             where agent_id = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![agent_id.as_ref()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<(String, String, String, String, String)> =
            rows.collect::<Result<_, _>>()?;

        let mut experiences = Vec::new();
        for row in raw_rows {
            let refs = self.collect_experience_refs(&row.0)?;
            experiences.push(Experience::construct_from_db(row, refs)?);
        }
        Ok(experiences)
    }

    pub fn list_experiences_by_sensation(
        &self,
        sensation: impl AsRef<str>,
    ) -> Result<Vec<Identity<ExperienceId, Experience>>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, sensation, description, created_at from experience \
             where sensation = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![sensation.as_ref()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<(String, String, String, String, String)> =
            rows.collect::<Result<_, _>>()?;

        let mut experiences = Vec::new();
        for row in raw_rows {
            let refs = self.collect_experience_refs(&row.0)?;
            experiences.push(Experience::construct_from_db(row, refs)?);
        }
        Ok(experiences)
    }

    pub fn get_experience_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Option<Identity<ExperienceId, Experience>>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, agent_id, sensation, description, created_at from experience where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        );

        match result {
            Ok(row) => {
                let refs = self.collect_experience_refs(&row.0)?;
                Ok(Some(Experience::construct_from_db(row, refs)?))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn add_experience_ref(
        &self,
        experience_id: impl AsRef<str>,
        record_ref: &RecordRef,
        created_at: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        match record_ref {
            RecordRef::Identified(r) => {
                self.conn.execute(
                    "insert or ignore into experience_ref \
                     (experience_id, record_id, record_kind, role, created_at) \
                     values (?1, ?2, ?3, ?4, ?5)",
                    params![
                        experience_id.as_ref(),
                        r.id.to_string(),
                        r.kind.to_string(),
                        r.role.as_ref().map(|l| l.as_str()),
                        created_at.as_ref()
                    ],
                )?;
            }
            RecordRef::Linked(r) => {
                self.conn.execute(
                    "insert or ignore into experience_ref \
                     (experience_id, link, role, created_at) \
                     values (?1, ?2, ?3, ?4)",
                    params![
                        experience_id.as_ref(),
                        r.link.to_string(),
                        r.role.as_ref().map(|l| l.as_str()),
                        created_at.as_ref()
                    ],
                )?;
            }
        }
        Ok(())
    }

    fn collect_experience_refs(
        &self,
        experience_id: &str,
    ) -> Result<Vec<RecordRef>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select record_id, record_kind, role, link \
             from experience_ref where experience_id = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![experience_id], |row| {
            let record_id: Option<String> = row.get(0)?;
            let record_kind: Option<String> = row.get(1)?;
            let role: Option<String> = row.get(2)?;
            let link: Option<String> = row.get(3)?;
            Ok((record_id, record_kind, role, link))
        })?;

        let mut refs = Vec::new();
        for row in rows {
            let (record_id, record_kind, role, link) = row?;
            let record_ref = if let Some(link_str) = link {
                let link = link_str.parse().map_err(RecordRefConstructionError::from)?;
                RecordRef::linked(link, role.map(Label::new))
            } else {
                let id_str = record_id.unwrap_or_default();
                let kind_str = record_kind.unwrap_or_default();
                IdentifiedRef::construct_from_db((id_str, kind_str, role))?
            };
            refs.push(record_ref);
        }
        Ok(refs)
    }

    pub fn update_experience_description(
        &self,
        id: impl AsRef<str>,
        description: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "update experience set description = ?2 where id = ?1",
            params![id.as_ref(), description.as_ref()],
        )?;
        Ok(())
    }

    pub fn reset_experiences(&self) -> Result<(), DatabaseError> {
        self.conn
            .execute_batch("delete from experience_ref; delete from experience")?;
        Ok(())
    }

    pub fn reset_experience_refs(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from experience_ref")?;
        Ok(())
    }

    // -- Blob operations (content-addressable store) --

    pub fn put_blob(
        &self,
        hash: impl AsRef<str>,
        data: &[u8],
        original_size: usize,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into blob (hash, data, size) values (?1, ?2, ?3)",
            params![hash.as_ref(), data, original_size as i64],
        )?;
        Ok(())
    }

    pub fn get_blob(
        &self,
        hash: impl AsRef<str>,
    ) -> Result<Option<(Vec<u8>, usize)>, DatabaseError> {
        let result = self.conn.query_row(
            "select data, size from blob where hash = ?1",
            params![hash.as_ref()],
            |row| {
                let data: Vec<u8> = row.get(0)?;
                let size: i64 = row.get(1)?;
                Ok((data, size as usize))
            },
        );

        match result {
            Ok(blob) => Ok(Some(blob)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    // -- Storage operations (projection table) --

    pub fn set_storage(
        &self,
        key: impl AsRef<str>,
        description: impl AsRef<str>,
        hash: impl AsRef<str>,
        link: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into storage (key, description, hash, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(key) do update set \
             description = excluded.description, hash = excluded.hash, \
             link = excluded.link",
            params![
                key.as_ref(),
                description.as_ref(),
                hash.as_ref(),
                link.as_ref()
            ],
        )?;
        Ok(())
    }

    pub fn remove_storage(&self, key: impl AsRef<str>) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from storage where key = ?1", params![key.as_ref()])?;
        Ok(())
    }

    pub fn get_storage(&self, key: impl AsRef<str>) -> Result<Option<StorageEntry>, DatabaseError> {
        let result = self.conn.query_row(
            "select key, description, hash from storage where key = ?1",
            params![key.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(StorageEntry::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_storage(&self) -> Result<Vec<StorageEntry>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("select key, description, hash from storage order by key")?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(StorageEntry::construct_from_db(row?));
        }
        Ok(entries)
    }

    pub fn get_storage_by_link(
        &self,
        link: impl AsRef<str>,
    ) -> Result<Option<StorageEntry>, DatabaseError> {
        let result = self.conn.query_row(
            "select key, description, hash from storage where link = ?1",
            params![link.as_ref()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        );

        match result {
            Ok(row) => Ok(Some(StorageEntry::construct_from_db(row))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn reset_storage(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from storage")?;
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
