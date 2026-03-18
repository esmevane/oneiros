use rusqlite::{Connection, params};

use crate::events::Events;
use crate::store::{Projection, StoreError, StoredEvent};

use crate::domains::agent::events::AgentEvents;
use crate::domains::cognition::events::CognitionEvents;
use crate::domains::experience::events::ExperienceEvents;
use crate::domains::memory::events::MemoryEvents;

pub const PROJECTIONS: &[Projection] = &[Projection {
    name: "search",
    apply: |conn, event| apply(conn, event),
    reset: |conn| reset(conn),
}];

fn apply(conn: &Connection, event: &StoredEvent) -> Result<(), StoreError> {
    match &event.data {
        Events::Cognition(CognitionEvents::CognitionAdded(c)) => {
            conn.execute(
                "INSERT INTO search_index (kind, entity_id, content, agent) VALUES (?1, ?2, ?3, ?4)",
                params!["cognition", c.id, c.content, c.agent_id],
            )?;
        }
        Events::Memory(MemoryEvents::MemoryAdded(m)) => {
            conn.execute(
                "INSERT INTO search_index (kind, entity_id, content, agent) VALUES (?1, ?2, ?3, ?4)",
                params!["memory", m.id, m.content, m.agent_id],
            )?;
        }
        Events::Agent(AgentEvents::AgentCreated(a)) => {
            conn.execute(
                "INSERT INTO search_index (kind, entity_id, content, agent) VALUES (?1, ?2, ?3, ?4)",
                params!["agent", a.id, format!("{} {}", a.name, a.description), a.name],
            )?;
        }
        Events::Experience(ExperienceEvents::ExperienceCreated(e)) => {
            conn.execute(
                "INSERT INTO search_index (kind, entity_id, content, agent) VALUES (?1, ?2, ?3, ?4)",
                params!["experience", e.id, e.description, e.agent_id],
            )?;
        }
        _ => {}
    }
    Ok(())
}

fn reset(conn: &Connection) -> Result<(), StoreError> {
    conn.execute("DELETE FROM search_index", [])?;
    Ok(())
}
