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

    /// Read all events in chronological order, with causal tiebreaking.
    ///
    /// Events are ordered by `timestamp ASC`. Within the same timestamp,
    /// seed/vocabulary events sort first (they define names referenced by
    /// other events), then agent lifecycle, then everything else. This
    /// ensures FK dependencies are satisfied during replay.
    pub fn read_events(&self) -> Result<Vec<EventRow>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, data FROM events
             ORDER BY timestamp ASC,
             CASE json_extract(meta, '$.type')
                 WHEN 'texture-set'   THEN 0
                 WHEN 'level-set'     THEN 0
                 WHEN 'persona-set'   THEN 0
                 WHEN 'sensation-set' THEN 0
                 WHEN 'nature-set'    THEN 0
                 WHEN 'agent-created' THEN 1
                 WHEN 'agent-updated' THEN 1
                 ELSE 2
             END ASC",
        )?;

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
        tenant_id: &TenantId,
        name: &TenantName,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let id_str = tenant_id.to_string();
        let link_str = link.to_string();
        self.conn.execute(
            "insert or ignore into tenant (id, name, link) values (?1, ?2, ?3)",
            params![&id_str, name.as_ref(), &link_str],
        )?;
        Ok(())
    }

    pub fn create_actor(
        &self,
        actor_id: &ActorId,
        tenant_id: &TenantId,
        name: &ActorName,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let actor_id_str = actor_id.to_string();
        let tenant_id_str = tenant_id.to_string();
        let link_str = link.to_string();
        self.conn.execute(
            "insert or ignore into actor (id, tenant_id, name, link) values (?1, ?2, ?3, ?4)",
            params![&actor_id_str, &tenant_id_str, name.as_ref(), &link_str],
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
        tenant_id: &TenantId,
        id: &BrainId,
    ) -> Result<Option<String>, DatabaseError> {
        let tenant_id_str = tenant_id.to_string();
        let id_str = id.to_string();
        let result = self.conn.query_row(
            "select path from brain where tenant_id = ?1 and id = ?2",
            params![&tenant_id_str, &id_str],
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
        tenant_id: &TenantId,
        name: &BrainName,
    ) -> Result<bool, DatabaseError> {
        let tenant_id_str = tenant_id.to_string();
        let count: i64 = self.conn.query_row(
            "select count(*) from brain where tenant_id = ?1 and name = ?2",
            params![&tenant_id_str, name.as_ref()],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn create_brain(
        &self,
        brain_id: &BrainId,
        tenant_id: &TenantId,
        name: &BrainName,
        path: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let brain_id_str = brain_id.to_string();
        let tenant_id_str = tenant_id.to_string();
        let link_str = link.to_string();
        self.conn.execute(
            "insert or ignore into brain (id, tenant_id, name, path, link) \
             values (?1, ?2, ?3, ?4, ?5)",
            params![&brain_id_str, &tenant_id_str, name.as_ref(), path, &link_str],
        )?;
        Ok(())
    }

    pub fn reset_brains(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from brain")?;
        Ok(())
    }

    pub fn get_actor_id(
        &self,
        tenant_id: &TenantId,
    ) -> Result<Option<String>, DatabaseError> {
        let tenant_id_str = tenant_id.to_string();
        let result = self.conn.query_row(
            "select id from actor where tenant_id = ?1 limit 1",
            params![&tenant_id_str],
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
        ticket_id: &TicketId,
        token: &str,
        created_by: &ActorId,
    ) -> Result<(), DatabaseError> {
        let id_str = ticket_id.to_string();
        let created_by_str = created_by.to_string();
        self.conn.execute(
            "insert into tickets (id, token, created_by) values (?1, ?2, ?3)",
            params![&id_str, token, &created_by_str],
        )?;
        Ok(())
    }

    pub fn validate_ticket(&self, token: &str) -> Result<bool, DatabaseError> {
        let result = self.conn.query_row(
            "select id from tickets \
             where token = ?1 \
             and revoked_on is null \
             and (expires_at is null or expires_at > strftime('%Y-%m-%dT%H:%M:%fZ', 'now')) \
             and (max_uses is null or uses < max_uses)",
            params![token],
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
        name: &PersonaName,
        description: &str,
        prompt: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let link_str = link.to_string();
        self.conn.execute(
            "insert into persona (name, description, prompt, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt, \
             link = excluded.link",
            params![name.as_ref(), description, prompt, &link_str],
        )?;
        Ok(())
    }

    pub fn remove_persona(&self, name: &PersonaName) -> Result<(), DatabaseError> {
        self.conn.execute(
            "delete from persona where name = ?1",
            params![name.as_ref()],
        )?;
        Ok(())
    }

    pub fn get_persona(&self, name: &PersonaName) -> Result<Option<Persona>, DatabaseError> {
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
        link: &Link,
    ) -> Result<Option<Persona>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select name, description, prompt from persona where link = ?1",
            params![&link_str],
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

    pub fn get_persona_by_key(
        &self,
        key: &Key<PersonaName, PersonaLink>,
    ) -> Result<Option<Persona>, DatabaseError> {
        if let Some(name) = key.try_id() {
            let result = self.get_persona(name)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_persona_by_link(&link);
        }
        Ok(None)
    }

    pub fn reset_personas(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from persona")?;
        Ok(())
    }

    // -- Texture operations --

    pub fn set_texture(
        &self,
        name: &TextureName,
        description: &str,
        prompt: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let link_str = link.to_string();
        self.conn.execute(
            "insert into texture (name, description, prompt, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt, \
             link = excluded.link",
            params![name.as_ref(), description, prompt, &link_str],
        )?;
        Ok(())
    }

    pub fn remove_texture(&self, name: &TextureName) -> Result<(), DatabaseError> {
        self.conn.execute(
            "delete from texture where name = ?1",
            params![name.as_ref()],
        )?;
        Ok(())
    }

    pub fn get_texture(&self, name: &TextureName) -> Result<Option<Texture>, DatabaseError> {
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
        link: &Link,
    ) -> Result<Option<Texture>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select name, description, prompt from texture where link = ?1",
            params![&link_str],
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

    pub fn get_texture_by_key(
        &self,
        key: &Key<TextureName, TextureLink>,
    ) -> Result<Option<Texture>, DatabaseError> {
        if let Some(name) = key.try_id() {
            let result = self.get_texture(name)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_texture_by_link(&link);
        }
        Ok(None)
    }

    pub fn reset_textures(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from texture")?;
        Ok(())
    }

    // -- Level operations --

    pub fn set_level(
        &self,
        name: &LevelName,
        description: &str,
        prompt: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let link_str = link.to_string();
        self.conn.execute(
            "insert into level (name, description, prompt, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt, \
             link = excluded.link",
            params![name.as_ref(), description, prompt, &link_str],
        )?;
        Ok(())
    }

    pub fn remove_level(&self, name: &LevelName) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from level where name = ?1", params![name.as_ref()])?;
        Ok(())
    }

    pub fn get_level(&self, name: &LevelName) -> Result<Option<Level>, DatabaseError> {
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

    pub fn get_level_by_link(&self, link: &Link) -> Result<Option<Level>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select name, description, prompt from level where link = ?1",
            params![&link_str],
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

    pub fn get_level_by_key(
        &self,
        key: &Key<LevelName, LevelLink>,
    ) -> Result<Option<Level>, DatabaseError> {
        if let Some(name) = key.try_id() {
            let result = self.get_level(name)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_level_by_link(&link);
        }
        Ok(None)
    }

    pub fn reset_levels(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from level")?;
        Ok(())
    }

    // -- Agent operations --

    pub fn create_agent_record(
        &self,
        id: &AgentId,
        name: &AgentName,
        persona: &PersonaName,
        description: &str,
        prompt: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let id_str = id.to_string();
        let link_str = link.to_string();
        self.conn.execute(
            "insert or ignore into agent (id, name, persona, description, prompt, link) \
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![&id_str, name.as_ref(), persona.as_ref(), description, prompt, &link_str],
        )?;
        Ok(())
    }

    pub fn update_agent(
        &self,
        name: &AgentName,
        persona: &PersonaName,
        description: &str,
        prompt: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let link_str = link.to_string();
        self.conn.execute(
            "update agent set persona = ?2, description = ?3, prompt = ?4, link = ?5 \
             where name = ?1",
            params![name.as_ref(), persona.as_ref(), description, prompt, &link_str],
        )?;
        Ok(())
    }

    pub fn remove_agent(&self, name: &AgentName) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from agent where name = ?1", params![name.as_ref()])?;
        Ok(())
    }

    pub fn get_agent(
        &self,
        name: &AgentName,
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
        link: &Link,
    ) -> Result<Option<Identity<AgentId, Agent>>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select id, name, persona, description, prompt from agent where link = ?1",
            params![&link_str],
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

    pub fn get_agent_by_key(
        &self,
        key: &Key<AgentName, AgentLink>,
    ) -> Result<Option<Identity<AgentId, Agent>>, DatabaseError> {
        if let Some(name) = key.try_id() {
            let result = self.get_agent(name)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_agent_by_link(&link);
        }
        Ok(None)
    }

    pub fn agent_name_exists(&self, name: &AgentName) -> Result<bool, DatabaseError> {
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
        id: &CognitionId,
        agent_id: &AgentId,
        texture: &TextureName,
        content: &str,
        created_at: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let id_str = id.to_string();
        let agent_id_str = agent_id.to_string();
        let link_str = link.to_string();
        self.conn.execute(
            "insert or ignore into cognition (id, agent_id, texture, content, created_at, link) \
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![&id_str, &agent_id_str, texture.as_ref(), content, created_at, &link_str],
        )?;
        Ok(())
    }

    pub fn get_cognition(
        &self,
        id: &CognitionId,
    ) -> Result<Option<Identity<CognitionId, Cognition>>, DatabaseError> {
        let id_str = id.to_string();
        let result = self.conn.query_row(
            "select id, agent_id, texture, content, created_at from cognition where id = ?1",
            params![&id_str],
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
        agent_id: &AgentId,
    ) -> Result<Vec<Identity<CognitionId, Cognition>>, DatabaseError> {
        let agent_id_str = agent_id.to_string();
        let mut stmt = self.conn.prepare(
            "select id, agent_id, texture, content, created_at from cognition \
             where agent_id = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![&agent_id_str], |row| {
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
        texture: &TextureName,
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
        agent_id: &AgentId,
        texture: &TextureName,
    ) -> Result<Vec<Identity<CognitionId, Cognition>>, DatabaseError> {
        let agent_id_str = agent_id.to_string();
        let mut stmt = self.conn.prepare(
            "select id, agent_id, texture, content, created_at from cognition \
             where agent_id = ?1 and texture = ?2 order by rowid",
        )?;

        let rows = stmt.query_map(params![&agent_id_str, texture.as_ref()], |row| {
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
        link: &Link,
    ) -> Result<Option<Identity<CognitionId, Cognition>>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select id, agent_id, texture, content, created_at from cognition where link = ?1",
            params![&link_str],
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

    pub fn get_cognition_by_key(
        &self,
        key: &Key<CognitionId, CognitionLink>,
    ) -> Result<Option<Identity<CognitionId, Cognition>>, DatabaseError> {
        if let Some(id) = key.try_id() {
            let result = self.get_cognition(id)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_cognition_by_link(&link);
        }
        Ok(None)
    }

    pub fn reset_cognitions(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from cognition")?;
        Ok(())
    }

    // -- Memory operations --

    pub fn add_memory(
        &self,
        id: &MemoryId,
        agent_id: &AgentId,
        level: &LevelName,
        content: &str,
        created_at: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let id_str = id.to_string();
        let agent_id_str = agent_id.to_string();
        let link_str = link.to_string();
        self.conn.execute(
            "insert or ignore into memory (id, agent_id, level, content, created_at, link) \
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![&id_str, &agent_id_str, level.as_ref(), content, created_at, &link_str],
        )?;
        Ok(())
    }

    pub fn get_memory(
        &self,
        id: &MemoryId,
    ) -> Result<Option<Identity<MemoryId, Memory>>, DatabaseError> {
        let id_str = id.to_string();
        let result = self.conn.query_row(
            "select id, agent_id, level, content, created_at from memory where id = ?1",
            params![&id_str],
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
        agent_id: &AgentId,
    ) -> Result<Vec<Identity<MemoryId, Memory>>, DatabaseError> {
        let agent_id_str = agent_id.to_string();
        let mut stmt = self.conn.prepare(
            "select id, agent_id, level, content, created_at from memory \
             where agent_id = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![&agent_id_str], |row| {
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
        level: &LevelName,
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
        agent_id: &AgentId,
        level: &LevelName,
    ) -> Result<Vec<Identity<MemoryId, Memory>>, DatabaseError> {
        let agent_id_str = agent_id.to_string();
        let mut stmt = self.conn.prepare(
            "select id, agent_id, level, content, created_at from memory \
             where agent_id = ?1 and level = ?2 order by rowid",
        )?;

        let rows = stmt.query_map(params![&agent_id_str, level.as_ref()], |row| {
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
        link: &Link,
    ) -> Result<Option<Identity<MemoryId, Memory>>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select id, agent_id, level, content, created_at from memory where link = ?1",
            params![&link_str],
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

    pub fn get_memory_by_key(
        &self,
        key: &Key<MemoryId, MemoryLink>,
    ) -> Result<Option<Identity<MemoryId, Memory>>, DatabaseError> {
        if let Some(id) = key.try_id() {
            let result = self.get_memory(id)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_memory_by_link(&link);
        }
        Ok(None)
    }

    pub fn reset_memories(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from memory")?;
        Ok(())
    }

    // -- Sensation operations --

    pub fn set_sensation(
        &self,
        name: &SensationName,
        description: &str,
        prompt: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let link_str = link.to_string();
        self.conn.execute(
            "insert into sensation (name, description, prompt, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt, \
             link = excluded.link",
            params![name.as_ref(), description, prompt, &link_str],
        )?;
        Ok(())
    }

    pub fn remove_sensation(&self, name: &SensationName) -> Result<(), DatabaseError> {
        self.conn.execute(
            "delete from sensation where name = ?1",
            params![name.as_ref()],
        )?;
        Ok(())
    }

    pub fn get_sensation(&self, name: &SensationName) -> Result<Option<Sensation>, DatabaseError> {
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
        link: &Link,
    ) -> Result<Option<Sensation>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select name, description, prompt from sensation where link = ?1",
            params![&link_str],
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

    pub fn get_sensation_by_key(
        &self,
        key: &Key<SensationName, SensationLink>,
    ) -> Result<Option<Sensation>, DatabaseError> {
        if let Some(name) = key.try_id() {
            let result = self.get_sensation(name)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_sensation_by_link(&link);
        }
        Ok(None)
    }

    pub fn reset_sensations(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from sensation")?;
        Ok(())
    }

    // -- Nature operations --

    pub fn set_nature(
        &self,
        name: &NatureName,
        description: &str,
        prompt: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let link_str = link.to_string();
        self.conn.execute(
            "insert into nature (name, description, prompt, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt, \
             link = excluded.link",
            params![name.as_ref(), description, prompt, &link_str],
        )?;
        Ok(())
    }

    pub fn remove_nature(&self, name: &NatureName) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from nature where name = ?1", params![name.as_ref()])?;
        Ok(())
    }

    pub fn get_nature(&self, name: &NatureName) -> Result<Option<Nature>, DatabaseError> {
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
        link: &Link,
    ) -> Result<Option<Nature>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select name, description, prompt from nature where link = ?1",
            params![&link_str],
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

    pub fn get_nature_by_key(
        &self,
        key: &Key<NatureName, NatureLink>,
    ) -> Result<Option<Nature>, DatabaseError> {
        if let Some(name) = key.try_id() {
            let result = self.get_nature(name)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_nature_by_link(&link);
        }
        Ok(None)
    }

    pub fn reset_natures(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from nature")?;
        Ok(())
    }

    // -- Connection operations --

    pub fn create_connection(
        &self,
        id: &ConnectionId,
        nature: &NatureName,
        from_link: &Link,
        to_link: &Link,
        created_at: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let id_str = id.to_string();
        let from_link_str = from_link.to_string();
        let to_link_str = to_link.to_string();
        let link_str = link.to_string();
        self.conn.execute(
            "insert or ignore into connection (id, nature, from_link, to_link, created_at, link) \
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![&id_str, nature.as_ref(), &from_link_str, &to_link_str, created_at, &link_str],
        )?;
        Ok(())
    }

    pub fn get_connection(
        &self,
        id: &ConnectionId,
    ) -> Result<Option<Identity<ConnectionId, oneiros_model::Connection>>, DatabaseError> {
        let id_str = id.to_string();
        let result = self.conn.query_row(
            "select id, nature, from_link, to_link, created_at from connection where id = ?1",
            params![&id_str],
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
        nature: &NatureName,
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
        link: &Link,
    ) -> Result<Vec<Identity<ConnectionId, oneiros_model::Connection>>, DatabaseError> {
        let link_str = link.to_string();
        let mut stmt = self.conn.prepare(
            "select id, nature, from_link, to_link, created_at from connection \
             where from_link = ?1 or to_link = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![&link_str], |row| {
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
        link: &Link,
    ) -> Result<Option<Identity<ConnectionId, oneiros_model::Connection>>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select id, nature, from_link, to_link, created_at from connection where link = ?1",
            params![&link_str],
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

    pub fn get_connection_by_key(
        &self,
        key: &Key<ConnectionId, ConnectionLink>,
    ) -> Result<Option<Identity<ConnectionId, oneiros_model::Connection>>, DatabaseError> {
        if let Some(id) = key.try_id() {
            let result = self.get_connection(id)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_connection_by_link(&link);
        }
        Ok(None)
    }

    pub fn remove_connection(&self, id: &ConnectionId) -> Result<(), DatabaseError> {
        let id_str = id.to_string();
        self.conn
            .execute("delete from connection where id = ?1", params![&id_str])?;
        Ok(())
    }

    pub fn reset_connections(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from connection")?;
        Ok(())
    }

    // -- Experience operations --

    pub fn add_experience(
        &self,
        id: &ExperienceId,
        agent_id: &AgentId,
        sensation: &SensationName,
        description: &str,
        created_at: &str,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let id_str = id.to_string();
        let agent_id_str = agent_id.to_string();
        let link_str = link.to_string();
        self.conn.execute(
            "insert or ignore into experience (id, agent_id, sensation, description, created_at, link) \
             values (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                &id_str,
                &agent_id_str,
                sensation.as_ref(),
                description,
                created_at,
                &link_str
            ],
        )?;
        Ok(())
    }

    pub fn get_experience(
        &self,
        id: &ExperienceId,
    ) -> Result<Option<Identity<ExperienceId, Experience>>, DatabaseError> {
        let id_str = id.to_string();
        let result = self.conn.query_row(
            "select id, agent_id, sensation, description, created_at from experience where id = ?1",
            params![&id_str],
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
                let refs = self.collect_experience_refs(&id_str)?;
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
        agent_id: &AgentId,
    ) -> Result<Vec<Identity<ExperienceId, Experience>>, DatabaseError> {
        let agent_id_str = agent_id.to_string();
        let mut stmt = self.conn.prepare(
            "select id, agent_id, sensation, description, created_at from experience \
             where agent_id = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![&agent_id_str], |row| {
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
        sensation: &SensationName,
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
        link: &Link,
    ) -> Result<Option<Identity<ExperienceId, Experience>>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select id, agent_id, sensation, description, created_at from experience where link = ?1",
            params![&link_str],
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

    pub fn get_experience_by_key(
        &self,
        key: &Key<ExperienceId, ExperienceLink>,
    ) -> Result<Option<Identity<ExperienceId, Experience>>, DatabaseError> {
        if let Some(id) = key.try_id() {
            let result = self.get_experience(id)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_experience_by_link(&link);
        }
        Ok(None)
    }

    pub fn add_experience_ref(
        &self,
        experience_id: &ExperienceId,
        entity_ref: &EntityRef,
        created_at: &str,
    ) -> Result<(), DatabaseError> {
        let experience_id_str = experience_id.to_string();
        self.conn.execute(
            "insert or ignore into experience_ref \
             (experience_id, record_id, link, role, created_at) \
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                &experience_id_str,
                entity_ref.id().map(|id| id.to_string()),
                entity_ref.link().map(|l| l.to_string()),
                entity_ref.role().map(|l| l.as_str()),
                created_at
            ],
        )?;
        Ok(())
    }

    fn collect_experience_refs(
        &self,
        experience_id: &str,
    ) -> Result<Vec<EntityRef>, DatabaseError> {
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
            refs.push(EntityRef::construct_from_db(record_id, record_kind, role, link)?);
        }
        Ok(refs)
    }

    pub fn update_experience_description(
        &self,
        id: &ExperienceId,
        description: &str,
    ) -> Result<(), DatabaseError> {
        let id_str = id.to_string();
        self.conn.execute(
            "update experience set description = ?2 where id = ?1",
            params![&id_str, description],
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
        hash: &ContentHash,
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
        hash: &ContentHash,
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
        key: &StorageKey,
        description: &str,
        hash: &ContentHash,
        link: &Link,
    ) -> Result<(), DatabaseError> {
        let link_str = link.to_string();
        self.conn.execute(
            "insert into storage (key, description, hash, link) \
             values (?1, ?2, ?3, ?4) \
             on conflict(key) do update set \
             description = excluded.description, hash = excluded.hash, \
             link = excluded.link",
            params![
                key.as_ref(),
                description,
                hash.as_ref(),
                &link_str
            ],
        )?;
        Ok(())
    }

    pub fn remove_storage(&self, key: &StorageKey) -> Result<(), DatabaseError> {
        self.conn
            .execute("delete from storage where key = ?1", params![key.as_ref()])?;
        Ok(())
    }

    pub fn get_storage(&self, key: &StorageKey) -> Result<Option<StorageEntry>, DatabaseError> {
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
        link: &Link,
    ) -> Result<Option<StorageEntry>, DatabaseError> {
        let link_str = link.to_string();
        let result = self.conn.query_row(
            "select key, description, hash from storage where link = ?1",
            params![&link_str],
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

    pub fn get_storage_by_key(
        &self,
        key: &Key<StorageKey, StorageLink>,
    ) -> Result<Option<StorageEntry>, DatabaseError> {
        if let Some(storage_key) = key.try_id() {
            let result = self.get_storage(storage_key)?;
            if result.is_some() {
                return Ok(result);
            }
        }
        if let Some(link) = key.try_link() {
            let link: Link = link.clone().into();
            return self.get_storage_by_link(&link);
        }
        Ok(None)
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

    /// Replay an event: insert it into the log and run projections best-effort.
    ///
    /// Unlike `log_event`, projection failures are non-fatal. This matches
    /// the original system's behavior where events commit before projections
    /// run — a projection failure in the original leaves the event logged
    /// but the projection unapplied. Returns `Ok(None)` on full success,
    /// `Ok(Some(err))` if the event was logged but a projection failed.
    pub fn replay_event(
        &self,
        data: &Value,
        projections: &[Projection],
    ) -> Result<Option<DatabaseError>, DatabaseError> {
        self.create_event(data)?;

        match self.run_projections(projections, data) {
            Ok(()) => Ok(None),
            Err(e) => Ok(Some(e)),
        }
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
