use oneiros_model::*;
use rusqlite::{Connection, functions::FunctionFlags, params};
use serde_json::Value;
use std::path::Path;
use uuid::Uuid;

use crate::*;

/// Strip characters that are FTS5 query operators but not useful for
/// free-text search. Preserves `*` (prefix), `"` (phrase), and boolean
/// keywords (AND, OR, NOT, NEAR).
fn sanitize_fts5_query(query: &str) -> String {
    query
        .chars()
        .map(|c| match c {
            '.' | ':' | '{' | '}' | '(' | ')' | '^' => ' ',
            _ => c,
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
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
    pub fn read_events(&self) -> Result<Vec<Event>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "SELECT json_object('id', id, 'timestamp', timestamp, 'data', json(data)) FROM events
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
            let raw_event: String = row.get(0)?;
            Ok(raw_event)
        })?;

        let mut events = Vec::new();

        for row in rows {
            let raw = row?;

            match serde_json::from_str::<Event>(&raw) {
                Ok(event) => events.push(event),
                Err(error) => {
                    eprintln!("Skipping malformed event: {error}");
                    continue;
                }
            }
        }

        Ok(events)
    }

    pub fn get_event(&self, id: &EventId) -> Result<Option<Event>, DatabaseError> {
        enum Attempt {
            Success(Event),
            Failure(serde_json::Error),
        }

        let result = self.conn.query_row(
            "select json_object('id', id, 'timestamp', timestamp, 'data', data) from events where id = ?1",
            params![id.to_string()],
            |row| {
                let raw_event: String = row.get(0)?;
                match serde_json::from_str::<Event>(&raw_event) {
                    Ok(event) => Ok(Attempt::Success(event)),
                    Err(error) => Ok(Attempt::Failure(error)),
                }
            }
        );

        match result {
            Ok(Attempt::Success(event)) => Ok(Some(event)),
            Ok(Attempt::Failure(error)) => Err(error)?,
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error)?,
        }
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

    pub fn create_tenant(
        &self,
        tenant_id: &TenantId,
        name: &TenantName,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into tenant (id, name) values (?1, ?2)",
            params![tenant_id.to_string(), name.as_ref()],
        )?;
        Ok(())
    }

    pub fn reset_actors(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from actor")?;
        Ok(())
    }

    pub fn create_actor(
        &self,
        actor_id: &ActorId,
        tenant_id: &TenantId,
        name: &ActorName,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into actor (id, tenant_id, name) values (?1, ?2, ?3)",
            params![actor_id.to_string(), tenant_id.to_string(), name.as_ref(),],
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
    ) -> Result<BrainId, DatabaseError> {
        let id: String = self.conn.query_row(
            "select id from brain where tenant_id = ?1 and name = ?2",
            params![tenant_id.as_ref(), name.as_ref()],
            |row| row.get(0),
        )?;

        Ok(id.parse()?)
    }

    pub fn create_brain(
        &self,
        brain_id: &BrainId,
        tenant_id: &TenantId,
        name: &BrainName,
        path: &str,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into brain (id, tenant_id, name, path) \
             values (?1, ?2, ?3, ?4)",
            params![
                brain_id.to_string(),
                tenant_id.to_string(),
                name.as_ref(),
                path,
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
        name: &PersonaName,
        description: &Description,
        prompt: &Prompt,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into persona (name, description, prompt) \
             values (?1, ?2, ?3) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt",
            params![name.as_ref(), description.as_str(), prompt.as_str()],
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

    pub fn reset_personas(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from persona")?;
        Ok(())
    }

    // -- Texture operations --

    pub fn set_texture(
        &self,
        name: &TextureName,
        description: &Description,
        prompt: &Prompt,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into texture (name, description, prompt) \
             values (?1, ?2, ?3) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt",
            params![name.as_ref(), description.as_str(), prompt.as_str()],
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

    pub fn reset_textures(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from texture")?;
        Ok(())
    }

    // -- Level operations --

    pub fn set_level(
        &self,
        name: &LevelName,
        description: &Description,
        prompt: &Prompt,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into level (name, description, prompt) \
             values (?1, ?2, ?3) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt",
            params![name.as_ref(), description.as_str(), prompt.as_str()],
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
        description: &Description,
        prompt: &Prompt,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into agent (id, name, persona, description, prompt) \
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                id.to_string(),
                name.as_ref(),
                persona.as_ref(),
                description.as_str(),
                prompt.as_str(),
            ],
        )?;
        Ok(())
    }

    pub fn update_agent(
        &self,
        name: &AgentName,
        persona: &PersonaName,
        description: &Description,
        prompt: &Prompt,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "update agent set persona = ?2, description = ?3, prompt = ?4 \
             where name = ?1",
            params![
                name.as_ref(),
                persona.as_ref(),
                description.as_str(),
                prompt.as_str(),
            ],
        )?;
        Ok(())
    }

    pub fn remove_agent(&self, name: impl AsRef<str>) -> Result<(), DatabaseError> {
        let tx = self.conn.unchecked_transaction()?;
        let name = name.as_ref();

        tx.execute(
            "delete from cognition where agent_id in (select id from agent where name = ?1)",
            params![name],
        )?;
        tx.execute(
            "delete from memory where agent_id in (select id from agent where name = ?1)",
            params![name],
        )?;
        tx.execute(
            "delete from experience where agent_id in (select id from agent where name = ?1)",
            params![name],
        )?;
        tx.execute("delete from agent where name = ?1", params![name])?;
        tx.commit()?;

        Ok(())
    }

    pub fn get_agent(&self, name: impl AsRef<str>) -> Result<Option<Agent>, DatabaseError> {
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

    pub fn list_agents(&self) -> Result<Vec<Agent>, DatabaseError> {
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
        id: &CognitionId,
        agent_id: &AgentId,
        texture: &TextureName,
        content: &Content,
        created_at: &str,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into cognition (id, agent_id, texture, content, created_at) \
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                id.to_string(),
                agent_id.to_string(),
                texture.as_ref(),
                content.as_str(),
                created_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_cognition(&self, id: impl AsRef<str>) -> Result<Option<Cognition>, DatabaseError> {
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

    pub fn list_cognitions(&self) -> Result<Vec<Cognition>, DatabaseError> {
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
    ) -> Result<Vec<Cognition>, DatabaseError> {
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

    pub fn list_recent_cognitions_by_agent(
        &self,
        agent_id: &AgentId,
        limit: usize,
    ) -> Result<Vec<Cognition>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, texture, content, created_at from cognition \
             where agent_id = ?1 order by created_at desc limit ?2",
        )?;

        let rows = stmt.query_map(params![agent_id.to_string(), limit as i64], |row| {
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

    pub fn list_cognition_ids_by_agent(
        &self,
        agent_id: &AgentId,
    ) -> Result<Vec<CognitionId>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM cognition WHERE agent_id = ?1")?;

        let rows = stmt.query_map(params![agent_id.to_string()], |row| row.get::<_, String>(0))?;

        rows.map(|r| {
            r.map_err(DatabaseError::from)
                .and_then(|s| s.parse().map_err(DatabaseError::from))
        })
        .collect()
    }

    pub fn list_cognitions_by_texture(
        &self,
        texture: impl AsRef<str>,
    ) -> Result<Vec<Cognition>, DatabaseError> {
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
    ) -> Result<Vec<Cognition>, DatabaseError> {
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
        content: &Content,
        created_at: &str,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into memory (id, agent_id, level, content, created_at) \
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                id.to_string(),
                agent_id.to_string(),
                level.as_ref(),
                content.as_str(),
                created_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_memory(&self, id: impl AsRef<str>) -> Result<Option<Memory>, DatabaseError> {
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

    pub fn list_memories(&self) -> Result<Vec<Memory>, DatabaseError> {
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
    ) -> Result<Vec<Memory>, DatabaseError> {
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

    pub fn list_memory_ids_by_agent(
        &self,
        agent_id: &AgentId,
    ) -> Result<Vec<MemoryId>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM memory WHERE agent_id = ?1")?;

        let rows = stmt.query_map(params![agent_id.to_string()], |row| row.get::<_, String>(0))?;

        rows.map(|r| {
            r.map_err(DatabaseError::from)
                .and_then(|s| s.parse().map_err(DatabaseError::from))
        })
        .collect()
    }

    pub fn list_memories_by_level(
        &self,
        level: impl AsRef<str>,
    ) -> Result<Vec<Memory>, DatabaseError> {
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
    ) -> Result<Vec<Memory>, DatabaseError> {
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

    pub fn list_recent_memories_by_agent(
        &self,
        agent_id: &AgentId,
        limit: usize,
    ) -> Result<Vec<Memory>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, level, content, created_at from memory \
             where agent_id = ?1 order by created_at desc limit ?2",
        )?;

        let rows = stmt.query_map(params![agent_id.to_string(), limit as i64], |row| {
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

    pub fn reset_memories(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from memory")?;
        Ok(())
    }

    // -- Sensation operations --

    pub fn set_sensation(
        &self,
        name: &SensationName,
        description: &Description,
        prompt: &Prompt,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into sensation (name, description, prompt) \
             values (?1, ?2, ?3) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt",
            params![name.as_ref(), description.as_str(), prompt.as_str()],
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

    pub fn reset_sensations(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from sensation")?;
        Ok(())
    }

    // -- Nature operations --

    pub fn set_nature(
        &self,
        name: &NatureName,
        description: &Description,
        prompt: &Prompt,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into nature (name, description, prompt) \
             values (?1, ?2, ?3) \
             on conflict(name) do update set \
             description = excluded.description, prompt = excluded.prompt",
            params![name.as_ref(), description.as_str(), prompt.as_str()],
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

    pub fn reset_natures(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from nature")?;
        Ok(())
    }

    // -- Connection operations --

    pub fn create_connection(
        &self,
        id: &ConnectionId,
        nature: &NatureName,
        from_ref: &Ref,
        to_ref: &Ref,
        created_at: &str,
    ) -> Result<(), DatabaseError> {
        let from_json = serde_json::to_string(from_ref)?;
        let to_json = serde_json::to_string(to_ref)?;

        self.conn.execute(
            "insert or ignore into connection (id, nature, from_ref, to_ref, created_at) \
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                id.to_string(),
                nature.as_ref(),
                from_json,
                to_json,
                created_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_connection(
        &self,
        id: impl AsRef<str>,
    ) -> Result<Option<oneiros_model::Connection>, DatabaseError> {
        let result = self.conn.query_row(
            "select id, nature, from_ref, to_ref, created_at from connection where id = ?1",
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

    pub fn list_connections(&self) -> Result<Vec<oneiros_model::Connection>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, nature, from_ref, to_ref, created_at from connection order by rowid",
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
    ) -> Result<Vec<oneiros_model::Connection>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, nature, from_ref, to_ref, created_at from connection \
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

    pub fn list_connections_by_ref(
        &self,
        entity_ref: &Ref,
    ) -> Result<Vec<oneiros_model::Connection>, DatabaseError> {
        let ref_json = serde_json::to_string(entity_ref)?;
        let mut stmt = self.conn.prepare(
            "select id, nature, from_ref, to_ref, created_at from connection \
             where from_ref = ?1 or to_ref = ?1 order by rowid",
        )?;

        let rows = stmt.query_map(params![ref_json], |row| {
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
        id: &ExperienceId,
        agent_id: &AgentId,
        sensation: &SensationName,
        description: &Description,
        created_at: &str,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert or ignore into experience (id, agent_id, sensation, description, created_at) \
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                id.to_string(),
                agent_id.to_string(),
                sensation.as_ref(),
                description.as_str(),
                created_at,
            ],
        )?;
        Ok(())
    }

    pub fn get_experience(&self, id: impl AsRef<str>) -> Result<Option<Experience>, DatabaseError> {
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
            Ok(row) => Ok(Some(Experience::construct_from_db(row)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_experiences(&self) -> Result<Vec<Experience>, DatabaseError> {
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
            experiences.push(Experience::construct_from_db(row)?);
        }
        Ok(experiences)
    }

    pub fn list_experiences_by_agent(
        &self,
        agent_id: impl AsRef<str>,
    ) -> Result<Vec<Experience>, DatabaseError> {
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
            experiences.push(Experience::construct_from_db(row)?);
        }
        Ok(experiences)
    }

    pub fn list_recent_experiences_by_agent(
        &self,
        agent_id: &AgentId,
        limit: usize,
    ) -> Result<Vec<Experience>, DatabaseError> {
        let mut stmt = self.conn.prepare(
            "select id, agent_id, sensation, description, created_at from experience \
             where agent_id = ?1 order by created_at desc limit ?2",
        )?;

        let rows = stmt.query_map(params![agent_id.to_string(), limit as i64], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let raw_rows: Vec<_> = rows.collect::<Result<_, _>>()?;
        let mut experiences = Vec::new();
        for row in raw_rows {
            experiences.push(Experience::construct_from_db(row)?);
        }
        Ok(experiences)
    }

    pub fn list_experience_ids_by_agent(
        &self,
        agent_id: &AgentId,
    ) -> Result<Vec<ExperienceId>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM experience WHERE agent_id = ?1")?;

        let rows = stmt.query_map(params![agent_id.to_string()], |row| row.get::<_, String>(0))?;

        rows.map(|r| {
            r.map_err(DatabaseError::from)
                .and_then(|s| s.parse().map_err(DatabaseError::from))
        })
        .collect()
    }

    pub fn list_experiences_by_sensation(
        &self,
        sensation: impl AsRef<str>,
    ) -> Result<Vec<Experience>, DatabaseError> {
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
            experiences.push(Experience::construct_from_db(row)?);
        }
        Ok(experiences)
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

    pub fn update_experience_sensation(
        &self,
        id: impl AsRef<str>,
        sensation: impl AsRef<str>,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "update experience set sensation = ?2 where id = ?1",
            params![id.as_ref(), sensation.as_ref()],
        )?;
        Ok(())
    }

    pub fn reset_experiences(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("delete from experience")?;
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
        key: &StorageKey,
        description: &Description,
        hash: &ContentHash,
    ) -> Result<(), DatabaseError> {
        self.conn.execute(
            "insert into storage (key, description, hash) \
             values (?1, ?2, ?3) \
             on conflict(key) do update set \
             description = excluded.description, hash = excluded.hash",
            params![key.as_ref(), description.as_str(), hash.as_ref(),],
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

    // -- Search expressions ---------------------------------------------------

    pub fn insert_expression(
        &self,
        resource_ref: &Ref,
        kind: &str,
        content: &str,
    ) -> Result<(), DatabaseError> {
        let ref_json = serde_json::to_string(resource_ref)?;
        self.conn.execute(
            "INSERT INTO expressions (resource_ref, kind, content) VALUES (?1, ?2, ?3)",
            params![ref_json, kind, content],
        )?;
        Ok(())
    }

    pub fn delete_expressions_by_ref(&self, resource_ref: &Ref) -> Result<(), DatabaseError> {
        let ref_json = serde_json::to_string(resource_ref)?;
        self.conn.execute(
            "DELETE FROM expressions WHERE resource_ref = ?1",
            params![ref_json],
        )?;
        Ok(())
    }

    pub fn search_expressions(&self, query: &str) -> Result<Vec<Expression>, DatabaseError> {
        let sanitized = sanitize_fts5_query(query);

        let mut stmt = self.conn.prepare(
            "SELECT e.resource_ref, e.kind, e.content \
             FROM expression_search s \
             JOIN expressions e ON e.id = s.rowid \
             WHERE expression_search MATCH ?1 \
             ORDER BY rank",
        )?;

        let rows = stmt.query_map(params![sanitized], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;

        let mut results = Vec::new();
        for row in rows {
            let (ref_json, kind, content) = row?;
            let resource_ref: Ref = serde_json::from_str(&ref_json)?;
            results.push(Expression {
                resource_ref,
                kind: Label::new(&kind),
                content: Content::new(&content),
            });
        }
        Ok(results)
    }

    pub fn reset_expressions(&self) -> Result<(), DatabaseError> {
        self.conn.execute_batch("DELETE FROM expressions")?;
        Ok(())
    }

    pub fn log_event(
        &self,
        data: impl serde::Serialize,
        projections: &[&[Projection]],
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

    pub fn import_event(&self, timestamp: &str, data: &Value) -> Result<(), DatabaseError> {
        let event_type = data["type"].as_str().unwrap_or("__unmarked");
        let meta = serde_json::json!({ "type": event_type });

        self.conn.execute(
            "insert into events (timestamp, data, meta) values (?1, ?2, ?3)",
            params![timestamp, data.to_string(), meta.to_string()],
        )?;

        Ok(())
    }

    pub fn replay(&self, projections: &[&[Projection]]) -> Result<usize, DatabaseError> {
        for group in projections.iter().rev() {
            for projection in group.iter().rev() {
                (projection.reset)(self)?;
            }
        }

        let mut stmt = self.conn.prepare(
            "SELECT data FROM events
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

        let mut count = 0;

        let rows = stmt.query_map([], |row| {
            let raw: String = row.get(0)?;
            Ok(raw)
        })?;

        for row in rows {
            let raw = row?;
            let event: Value = serde_json::from_str(&raw)?;

            self.run_projections(projections, &event)?;
            count += 1;
        }

        Ok(count)
    }

    fn run_projections(
        &self,
        projections: &[&[Projection]],
        event: &Value,
    ) -> Result<(), DatabaseError> {
        let Some(event_type) = event["type"].as_str() else {
            return Ok(());
        };

        let data = event["data"].clone();

        projections::project(self, projections, event_type, &data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_brain() -> (TempDir, Database) {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test-brain.db");
        let db = Database::create_brain_db(&db_path).unwrap();
        (temp, db)
    }

    #[test]
    fn insert_and_search_expression() {
        let (_temp, db) = setup_brain();
        let r = Ref::cognition(CognitionId::new());

        db.insert_expression(&r, "cognition-content", "the quick brown fox")
            .unwrap();

        let results = db.search_expressions("quick").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].resource_ref, r);
        assert_eq!(results[0].kind.as_ref(), "cognition-content");
        assert!(results[0].content.as_str().contains("quick brown fox"));
    }

    #[test]
    fn search_returns_empty_for_no_match() {
        let (_temp, db) = setup_brain();
        let r = Ref::cognition(CognitionId::new());

        db.insert_expression(&r, "cognition-content", "hello world")
            .unwrap();

        let results = db.search_expressions("zebra").unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn search_sanitizes_fts5_special_characters() {
        let (_temp, db) = setup_brain();
        let r = Ref::cognition(CognitionId::new());

        db.insert_expression(&r, "cognition-content", "governor process agent")
            .unwrap();

        // Periods in queries should not cause FTS5 syntax errors.
        let results = db.search_expressions("governor.process").unwrap();
        assert_eq!(results.len(), 1);

        // Colons and other operators should also be sanitized.
        let results = db.search_expressions("governor:process").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn sanitize_fts5_query_preserves_search_operators() {
        assert_eq!(sanitize_fts5_query("hello world"), "hello world");
        assert_eq!(sanitize_fts5_query("hello AND world"), "hello AND world");
        assert_eq!(sanitize_fts5_query("prefix*"), "prefix*");
        assert_eq!(sanitize_fts5_query("governor.process"), "governor process");
        assert_eq!(sanitize_fts5_query("col:term"), "col term");
        assert_eq!(sanitize_fts5_query("a.b:c{d}(e)"), "a b c d e");
    }

    #[test]
    fn delete_expressions_by_ref_removes_all_for_entity() {
        let (_temp, db) = setup_brain();
        let r1 = Ref::cognition(CognitionId::new());
        let r2 = Ref::cognition(CognitionId::new());

        db.insert_expression(&r1, "cognition-content", "alpha beta")
            .unwrap();
        db.insert_expression(&r1, "cognition-texture", "observation")
            .unwrap();
        db.insert_expression(&r2, "cognition-content", "alpha gamma")
            .unwrap();

        // Delete r1's expressions — r2 should survive.
        db.delete_expressions_by_ref(&r1).unwrap();

        let results = db.search_expressions("alpha").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].resource_ref, r2);
    }

    #[test]
    fn fts5_porter_stemming_matches_word_stems() {
        let (_temp, db) = setup_brain();
        let r = Ref::memory(MemoryId::new());

        db.insert_expression(&r, "memory-content", "running quickly through the gardens")
            .unwrap();

        // Porter stemmer: "run" should match "running", "garden" should match "gardens"
        let results = db.search_expressions("run").unwrap();
        assert_eq!(results.len(), 1);

        let results = db.search_expressions("garden").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn search_ranks_by_relevance() {
        let (_temp, db) = setup_brain();
        let r1 = Ref::cognition(CognitionId::new());
        let r2 = Ref::cognition(CognitionId::new());

        // r1 mentions "architecture" once among many words
        db.insert_expression(
            &r1,
            "cognition-content",
            "the project has many concerns including architecture and testing",
        )
        .unwrap();

        // r2 is focused on "architecture"
        db.insert_expression(
            &r2,
            "cognition-content",
            "architecture decisions shape the architecture of the system",
        )
        .unwrap();

        let results = db.search_expressions("architecture").unwrap();
        assert_eq!(results.len(), 2);
        // BM25 ranking: the more focused document should rank first
        assert_eq!(results[0].resource_ref, r2);
    }

    #[test]
    fn reset_expressions_clears_all() {
        let (_temp, db) = setup_brain();
        let r = Ref::cognition(CognitionId::new());

        db.insert_expression(&r, "cognition-content", "hello world")
            .unwrap();
        db.insert_expression(&r, "cognition-texture", "observation")
            .unwrap();

        db.reset_expressions().unwrap();

        let results = db.search_expressions("hello").unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn multiple_expressions_per_entity() {
        let (_temp, db) = setup_brain();
        let r = Ref::agent(AgentId::new());

        db.insert_expression(&r, "agent-name", "governor").unwrap();
        db.insert_expression(&r, "agent-description", "orchestration and routing")
            .unwrap();
        db.insert_expression(&r, "agent-prompt", "you are the governor process")
            .unwrap();

        let results = db.search_expressions("governor").unwrap();
        assert_eq!(results.len(), 2); // name + prompt both match

        let results = db.search_expressions("orchestration").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].kind.as_ref(), "agent-description");
    }
}
