//! Agent view — presentation authority for the agent domain.
//!
//! Owns the response and produces `Rendered<AgentResponse>` with
//! navigational hints.

use crate::*;

pub struct AgentView {
    response: AgentResponse,
}

impl AgentView {
    pub fn new(response: AgentResponse) -> Self {
        Self { response }
    }

    pub fn render(self) -> Rendered<AgentResponse> {
        match self.response {
            AgentResponse::AgentCreated(name) => {
                let prompt = Confirmation::new("Agent", name.to_string(), "created").to_string();
                let hints = HintSet::agent_created(
                    AgentCreatedHints::builder().agent(name.clone()).build(),
                );
                Rendered::new(AgentResponse::AgentCreated(name), prompt, String::new())
                    .with_hints(hints)
            }
            AgentResponse::AgentDetails(wrapped) => {
                let agent = &wrapped.data;
                let prompt = Detail::new(agent.name.to_string())
                    .field("persona:", agent.persona.to_string())
                    .field("description:", agent.description.to_string())
                    .field("prompt:", agent.prompt.to_string())
                    .to_string();
                Rendered::new(AgentResponse::AgentDetails(wrapped), prompt, String::new())
            }
            AgentResponse::Agents(listed) => {
                let mut table = Table::new(vec![
                    Column::key("name", "Name"),
                    Column::key("persona", "Persona"),
                    Column::key("description", "Description").max(60),
                ]);
                for wrapped in &listed.items {
                    let agent = &wrapped.data;
                    table.push_row(vec![
                        agent.name.to_string(),
                        agent.persona.to_string(),
                        agent.description.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                );
                Rendered::new(AgentResponse::Agents(listed), prompt, String::new())
            }
            AgentResponse::NoAgents => Rendered::new(
                AgentResponse::NoAgents,
                format!("{}", "No agents configured.".muted()),
                String::new(),
            ),
            AgentResponse::AgentUpdated(name) => {
                let prompt = Confirmation::new("Agent", name.to_string(), "updated").to_string();
                Rendered::new(AgentResponse::AgentUpdated(name), prompt, String::new())
            }
            AgentResponse::AgentRemoved(name) => {
                let prompt = Confirmation::new("Agent", name.to_string(), "removed").to_string();
                Rendered::new(AgentResponse::AgentRemoved(name), prompt, String::new())
            }
        }
    }
}
