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
            index(conn, Ref::cognition(c.id), "cognition-content", &c.content, &c.agent_id)?;
        }
        Events::Memory(MemoryEvents::MemoryAdded(m)) => {
            index(conn, Ref::memory(m.id), "memory-content", &m.content, &m.agent_id)?;
        }
        Events::Agent(AgentEvents::AgentCreated(a)) => {
            let content = format!("{} {}", a.name, a.description);
            index(conn, Ref::agent(a.id), "agent-description", &content, &a.name)?;
        }
        Events::Experience(ExperienceEvents::ExperienceCreated(e)) => {
            index(conn, Ref::experience(e.id), "experience-description", &e.description, &e.agent_id)?;
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

fn reset(conn: &Connection) -> Result<(), EventError> {
    conn.execute("DELETE FROM search_index", [])?;
    Ok(())
}
