use rusqlite::{Connection, params};

use crate::*;

pub struct SearchRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SearchRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create virtual table if not exists search_index
             using fts5(resource_ref, kind, content, agent)",
        )?;
        Ok(())
    }

    // ── Projection handling ─────────────────────────────────────

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        match &event.data {
            Events::Cognition(CognitionEvents::CognitionAdded(c)) => {
                self.index(
                    Ref::cognition(c.id),
                    "cognition-content",
                    &c.content,
                    c.agent_id,
                )?;
            }
            Events::Memory(MemoryEvents::MemoryAdded(m)) => {
                self.index(Ref::memory(m.id), "memory-content", &m.content, m.agent_id)?;
            }
            Events::Agent(AgentEvents::AgentCreated(a)) => {
                let content = format!("{} {}", a.name, a.description);
                self.index(Ref::agent(a.id), "agent-description", &content, &a.name)?;
            }
            Events::Agent(AgentEvents::AgentUpdated(a)) => {
                self.remove_by_ref(&Ref::agent(a.id))?;
                let content = format!("{} {}", a.name, a.description);
                self.index(Ref::agent(a.id), "agent-description", &content, &a.name)?;
            }
            Events::Agent(AgentEvents::AgentRemoved(removed)) => {
                self.remove_by_agent(&removed.name)?;
            }
            Events::Experience(ExperienceEvents::ExperienceCreated(e)) => {
                self.index(
                    Ref::experience(e.id),
                    "experience-description",
                    &e.description,
                    e.agent_id,
                )?;
            }
            Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(update)) => {
                self.remove_by_ref(&Ref::experience(update.id))?;
                if let Ok(Some(exp)) = ExperienceRepo::new(self.conn).get(&update.id) {
                    self.index(
                        Ref::experience(update.id),
                        "experience-description",
                        &update.description,
                        exp.agent_id,
                    )?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM search_index", [])?;
        Ok(())
    }

    // ── Read queries ────────────────────────────────────────────

    pub fn search(
        &self,
        query: &str,
        agent: Option<&AgentId>,
    ) -> Result<Vec<Expression>, EventError> {
        let base =
            "select resource_ref, kind, content from search_index where search_index match ?1";

        match agent {
            Some(agent_filter) => {
                let sql = format!("{base} and agent = ?2 order by rank");
                let mut statement = self.conn.prepare(&sql)?;
                Ok(statement
                    .query_map(params![query, agent_filter.to_string()], Self::map_row)?
                    .collect::<Result<Vec<_>, _>>()?)
            }
            None => {
                let sql = format!("{base} order by rank");
                let mut statement = self.conn.prepare(&sql)?;
                Ok(statement
                    .query_map(params![query], Self::map_row)?
                    .collect::<Result<Vec<_>, _>>()?)
            }
        }
    }

    // ── Write operations ────────────────────────────────────────

    fn index(
        &self,
        resource_ref: Ref,
        kind: &str,
        content: impl std::fmt::Display,
        agent: impl std::fmt::Display,
    ) -> Result<(), EventError> {
        let ref_json = serde_json::to_string(&resource_ref)?;
        self.conn.execute(
            "INSERT INTO search_index (resource_ref, kind, content, agent) VALUES (?1, ?2, ?3, ?4)",
            params![ref_json, kind, content.to_string(), agent.to_string()],
        )?;
        Ok(())
    }

    fn remove_by_ref(&self, resource_ref: &Ref) -> Result<(), EventError> {
        let ref_json = serde_json::to_string(resource_ref)?;
        self.conn.execute(
            "DELETE FROM search_index WHERE resource_ref = ?1",
            params![ref_json],
        )?;
        Ok(())
    }

    fn remove_by_agent(&self, agent: &AgentName) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM search_index WHERE agent = ?1",
            params![agent.to_string()],
        )?;
        Ok(())
    }

    // ── Helpers ──────────────────────────────────────────────────

    fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Expression> {
        let ref_json: String = row.get(0)?;
        let resource_ref: Ref = serde_json::from_str(&ref_json).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?;

        Ok(Expression::builder()
            .resource_ref(resource_ref)
            .kind(row.get::<_, String>(1)?)
            .content(row.get::<_, String>(2)?)
            .build())
    }
}
