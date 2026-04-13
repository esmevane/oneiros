use rusqlite::params;

use crate::*;

/// Search projection store — projection lifecycle, write operations, and index management.
pub(crate) struct SearchStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SearchStore<'a> {
    pub(crate) fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    // ── Projection handling ─────────────────────────────────────

    pub(crate) fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create virtual table if not exists search_index
             using fts5(resource_ref, kind, content, agent)",
        )?;
        Ok(())
    }

    pub(crate) fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
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
                if let Ok(Some(exp)) = ExperienceStore::new(self.conn).get(&update.id) {
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

    pub(crate) fn reset(&self) -> Result<(), EventError> {
        self.conn.execute("DELETE FROM search_index", [])?;
        Ok(())
    }

    // ── Write operations ─────────────────────────────────────────

    pub(crate) fn index(
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

    pub(crate) fn remove_by_ref(&self, resource_ref: &Ref) -> Result<(), EventError> {
        let ref_json = serde_json::to_string(resource_ref)?;
        self.conn.execute(
            "DELETE FROM search_index WHERE resource_ref = ?1",
            params![ref_json],
        )?;
        Ok(())
    }

    pub(crate) fn remove_by_agent(&self, agent: &AgentName) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM search_index WHERE agent = ?1",
            params![agent.to_string()],
        )?;
        Ok(())
    }
}
