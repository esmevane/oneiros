use rusqlite::{Connection, params};

use crate::store::{StoreError, StoredEvent};

use super::model::Agent;

/// Agent read model — queries, projection handling, and lifecycle.
pub struct AgentRepo<'a> {
    conn: &'a Connection,
}

impl<'a> AgentRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), StoreError> {
        match event.event_type.as_str() {
            "agent-created" => {
                let agent: Agent = serde_json::from_value(event.data.clone())?;
                self.create_record(&agent)?;
            }
            "agent-updated" => {
                let agent: Agent = serde_json::from_value(event.data.clone())?;
                self.update(&agent)?;
            }
            "agent-removed" => {
                if let Some(name) = event.data.get("name").and_then(|v| v.as_str()) {
                    self.remove(name)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM agents", [])?;
        Ok(())
    }

    pub fn migrate(&self) -> Result<(), StoreError> {
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

    pub fn get(&self, name: &str) -> Result<Option<Agent>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, persona, description, prompt FROM agents WHERE name = ?1")?;

        let result = stmt.query_row(params![name], |row| {
            Ok(Agent {
                id: row.get(0)?,
                name: row.get(1)?,
                persona: row.get(2)?,
                description: row.get(3)?,
                prompt: row.get(4)?,
            })
        });

        match result {
            Ok(agent) => Ok(Some(agent)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn list(&self) -> Result<Vec<Agent>, StoreError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, persona, description, prompt FROM agents ORDER BY name")?;

        let agents = stmt
            .query_map([], |row| {
                Ok(Agent {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    persona: row.get(2)?,
                    description: row.get(3)?,
                    prompt: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(agents)
    }

    pub fn name_exists(&self, name: &str) -> Result<bool, StoreError> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM agents WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ── Write operations (called by handle) ─────────────────────

    fn create_record(&self, agent: &Agent) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO agents (id, name, persona, description, prompt)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                agent.id,
                agent.name,
                agent.persona,
                agent.description,
                agent.prompt
            ],
        )?;
        Ok(())
    }

    fn update(&self, agent: &Agent) -> Result<(), StoreError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO agents (id, name, persona, description, prompt)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                agent.id,
                agent.name,
                agent.persona,
                agent.description,
                agent.prompt
            ],
        )?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<(), StoreError> {
        self.conn
            .execute("DELETE FROM agents WHERE name = ?1", params![name])?;
        Ok(())
    }
}
