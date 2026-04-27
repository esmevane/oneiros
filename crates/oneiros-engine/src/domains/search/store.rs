use rusqlite::params;

use crate::*;

/// Search projection store — projection lifecycle, write operations, and index management.
pub struct SearchStore<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SearchStore<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn migrate(&self) -> Result<(), EventError> {
        self.conn.execute_batch(
            "create virtual table if not exists search_index
             using fts5(resource_ref, kind, content, agent)",
        )?;
        Ok(())
    }

    pub fn handle(&self, event: &StoredEvent) -> Result<(), EventError> {
        match &event.data {
            Event::Known(Events::Cognition(CognitionEvents::CognitionAdded(cognition))) => {
                self.index(
                    Ref::cognition(cognition.id()),
                    "cognition-content",
                    cognition.content(),
                    cognition.agent_id(),
                )?;
            }
            Event::Known(Events::Memory(MemoryEvents::MemoryAdded(memory))) => {
                self.index(
                    Ref::memory(memory.id()),
                    "memory-content",
                    memory.content(),
                    memory.agent_id(),
                )?;
            }
            Event::Known(Events::Agent(AgentEvents::AgentCreated(agent))) => {
                let content = format!("{} {}", agent.name(), agent.description());
                self.index(
                    Ref::agent(agent.id()),
                    "agent-description",
                    &content,
                    agent.name(),
                )?;
            }
            Event::Known(Events::Agent(AgentEvents::AgentUpdated(agent))) => {
                self.remove_by_ref(&Ref::agent(agent.id()))?;
                let content = format!("{} {}", agent.name(), agent.description());
                self.index(
                    Ref::agent(agent.id()),
                    "agent-description",
                    &content,
                    agent.name(),
                )?;
            }
            Event::Known(Events::Agent(AgentEvents::AgentRemoved(removed))) => {
                self.remove_by_agent(removed.name())?;
            }
            Event::Known(Events::Experience(ExperienceEvents::ExperienceCreated(experience))) => {
                self.index(
                    Ref::experience(experience.id()),
                    "experience-description",
                    experience.description(),
                    experience.agent_id(),
                )?;
            }
            Event::Known(Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(
                update,
            ))) => {
                self.remove_by_ref(&Ref::experience(update.id()))?;
                if let Ok(Some(exp)) = ExperienceStore::new(self.conn).get(&update.id()) {
                    self.index(
                        Ref::experience(update.id()),
                        "experience-description",
                        update.description(),
                        exp.agent_id(),
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

    pub fn index(
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

    pub fn remove_by_ref(&self, resource_ref: &Ref) -> Result<(), EventError> {
        let ref_json = serde_json::to_string(resource_ref)?;
        self.conn.execute(
            "DELETE FROM search_index WHERE resource_ref = ?1",
            params![ref_json],
        )?;
        Ok(())
    }

    pub fn remove_by_agent(&self, agent: &AgentName) -> Result<(), EventError> {
        self.conn.execute(
            "DELETE FROM search_index WHERE agent = ?1",
            params![agent.to_string()],
        )?;
        Ok(())
    }
}
