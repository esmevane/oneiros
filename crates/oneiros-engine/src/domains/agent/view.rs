//! Agent view — presentation authority for the agent domain.
//!
//! Maps agent responses into shared view primitives (Table, Detail,
//! Confirmation). The domain knows its own shape; the rendering
//! layer decides how to display it.

use crate::*;

pub struct AgentView;

impl AgentView {
    /// Table of agents with standard columns.
    pub fn table(agents: &Listed<Response<Agent>>) -> Table {
        let mut table = Table::new(vec![
            Column::key("name", "Name"),
            Column::key("persona", "Persona"),
            Column::key("description", "Description").max(60),
        ]);

        for wrapped in &agents.items {
            let agent = &wrapped.data;
            table.push_row(vec![
                agent.name.to_string(),
                agent.persona.to_string(),
                agent.description.to_string(),
            ]);
        }

        table
    }

    /// Detail view for a single agent.
    pub fn detail(agent: &Agent) -> Detail {
        Detail::new(agent.name.to_string())
            .field("persona:", agent.persona.to_string())
            .field("description:", agent.description.to_string())
            .field("prompt:", agent.prompt.to_string())
    }

    /// Confirmation for a mutation.
    pub fn confirmed(verb: &str, name: &AgentName) -> Confirmation {
        Confirmation::new("Agent", name.to_string(), verb)
    }
}
