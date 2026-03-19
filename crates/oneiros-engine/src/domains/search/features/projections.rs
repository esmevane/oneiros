use rusqlite::{Connection, params};

use crate::*;

pub struct SearchProjections;

impl SearchProjections {
    pub const fn all(&self) -> &'static [Projection] {
        PROJECTIONS
    }
}

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "search",
    apply: |conn, event| apply(conn, event),
    reset: |conn| reset(conn),
}];

fn apply(conn: &Connection, event: &StoredEvent) -> Result<(), EventError> {
    match &event.data {
        Events::Cognition(CognitionEvents::CognitionAdded(c)) => {
            conn.execute(
                "INSERT INTO search_index (kind, entity_id, content, agent) VALUES (?1, ?2, ?3, ?4)",
                params!["cognition", c.id.to_string(), c.content.to_string(), c.agent_id.to_string()],
            )?;
        }
        Events::Memory(MemoryEvents::MemoryAdded(m)) => {
            conn.execute(
                "INSERT INTO search_index (kind, entity_id, content, agent) VALUES (?1, ?2, ?3, ?4)",
                params!["memory", m.id.to_string(), m.content.to_string(), m.agent_id.to_string()],
            )?;
        }
        Events::Agent(AgentEvents::AgentCreated(a)) => {
            conn.execute(
                "INSERT INTO search_index (kind, entity_id, content, agent) VALUES (?1, ?2, ?3, ?4)",
                params!["agent", a.id.to_string(), format!("{} {}", a.name, a.description), a.name.to_string()],
            )?;
        }
        Events::Experience(ExperienceEvents::ExperienceCreated(e)) => {
            conn.execute(
                "INSERT INTO search_index (kind, entity_id, content, agent) VALUES (?1, ?2, ?3, ?4)",
                params!["experience", e.id.to_string(), e.description.to_string(), e.agent_id.to_string()],
            )?;
        }
        _ => {}
    }
    Ok(())
}

fn reset(conn: &Connection) -> Result<(), EventError> {
    conn.execute("DELETE FROM search_index", [])?;
    Ok(())
}
