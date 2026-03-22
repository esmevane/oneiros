use rusqlite::{Connection, params};

use crate::*;

pub struct SearchProjections;

impl SearchProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

const PROJECTIONS: &[Projection] = &[Projection {
    name: "search",
    apply: |conn, event| apply(conn, event),
    reset: |conn| reset(conn),
}];

fn apply(conn: &Connection, event: &StoredEvent) -> Result<(), EventError> {
    match &event.data {
        Events::Cognition(CognitionEvents::CognitionAdded(c)) => {
            index(
                conn,
                Ref::cognition(c.id),
                "cognition-content",
                &c.content,
                &c.agent_id,
            )?;
        }
        Events::Memory(MemoryEvents::MemoryAdded(m)) => {
            index(
                conn,
                Ref::memory(m.id),
                "memory-content",
                &m.content,
                &m.agent_id,
            )?;
        }
        Events::Agent(AgentEvents::AgentCreated(a)) => {
            let content = format!("{} {}", a.name, a.description);
            index(
                conn,
                Ref::agent(a.id),
                "agent-description",
                &content,
                &a.name,
            )?;
        }
        Events::Agent(AgentEvents::AgentUpdated(a)) => {
            remove_by_ref(conn, &Ref::agent(a.id))?;
            let content = format!("{} {}", a.name, a.description);
            index(
                conn,
                Ref::agent(a.id),
                "agent-description",
                &content,
                &a.name,
            )?;
        }
        Events::Agent(AgentEvents::AgentRemoved(removed)) => {
            remove_by_agent(conn, &removed.name)?;
        }
        Events::Experience(ExperienceEvents::ExperienceCreated(e)) => {
            index(
                conn,
                Ref::experience(e.id),
                "experience-description",
                &e.description,
                &e.agent_id,
            )?;
        }
        Events::Experience(ExperienceEvents::ExperienceDescriptionUpdated(update)) => {
            remove_by_ref(conn, &Ref::experience(update.id))?;
            // Look up the agent from the experience record for the search index.
            if let Ok(Some(exp)) = ExperienceRepo::new(conn).get(&update.id) {
                index(
                    conn,
                    Ref::experience(update.id),
                    "experience-description",
                    &update.description,
                    &exp.agent_id,
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn index(
    conn: &Connection,
    resource_ref: Ref,
    kind: &str,
    content: impl std::fmt::Display,
    agent: impl std::fmt::Display,
) -> Result<(), EventError> {
    let ref_json = serde_json::to_string(&resource_ref)?;
    conn.execute(
        "INSERT INTO search_index (resource_ref, kind, content, agent) VALUES (?1, ?2, ?3, ?4)",
        params![ref_json, kind, content.to_string(), agent.to_string()],
    )?;
    Ok(())
}

fn remove_by_agent(conn: &Connection, agent: &AgentName) -> Result<(), EventError> {
    conn.execute(
        "DELETE FROM search_index WHERE agent = ?1",
        params![agent.to_string()],
    )?;
    Ok(())
}

fn remove_by_ref(conn: &Connection, resource_ref: &Ref) -> Result<(), EventError> {
    let ref_json = serde_json::to_string(resource_ref)?;
    conn.execute(
        "DELETE FROM search_index WHERE resource_ref = ?1",
        params![ref_json],
    )?;
    Ok(())
}

fn reset(conn: &Connection) -> Result<(), EventError> {
    conn.execute("DELETE FROM search_index", [])?;
    Ok(())
}
