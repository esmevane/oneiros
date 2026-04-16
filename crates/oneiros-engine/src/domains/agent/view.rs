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

    pub fn mcp(&self) -> McpResponse {
        let response = &self.response;
        match response {
            AgentResponse::AgentCreated(name) => McpResponse::new(format!("Agent created: {name}"))
                .hint_set(HintSet::agent_created(
                    AgentCreatedHints::builder().agent(name.clone()).build(),
                )),
            AgentResponse::AgentDetails(wrapped) => McpResponse::new(format!(
                "# {}\n\n**persona:** {}\n**description:** {}\n\n{}\n",
                wrapped.data.name,
                wrapped.data.persona,
                wrapped.data.description,
                wrapped.data.prompt
            ))
            .hint(Hint::inspect(
                ResourcePath::AgentCognitions(wrapped.data.name.clone()).uri(),
                "Browse cognitions",
            ))
            .hint(Hint::inspect(
                ResourcePath::AgentPressure(wrapped.data.name.clone()).uri(),
                "Check pressure",
            )),
            AgentResponse::Agents(listed) => {
                let mut md = format!("# Agents\n\n{} of {} total\n\n", listed.len(), listed.total);
                md.push_str("| Name | Persona | Description |\n");
                md.push_str("|------|---------|-------------|\n");
                for wrapped in &listed.items {
                    md.push_str(&format!(
                        "| {} | {} | {} |\n",
                        wrapped.data.name, wrapped.data.persona, wrapped.data.description
                    ));
                }
                McpResponse::new(md).hint(Hint::suggest(
                    "create-agent",
                    "Bring a new agent into the brain",
                ))
            }
            AgentResponse::NoAgents => McpResponse::new("# Agents\n\nNo agents configured.").hint(
                Hint::suggest("create-agent", "Bring a new agent into the brain"),
            ),
            AgentResponse::AgentUpdated(name) => McpResponse::new(format!("Agent updated: {name}"))
                .hint(Hint::inspect(
                    ResourcePath::Agent(name.clone()).uri(),
                    "View agent details",
                )),
            AgentResponse::AgentRemoved(name) => McpResponse::new(format!("Agent removed: {name}"))
                .hint(Hint::inspect(
                    ResourcePath::Agents.uri(),
                    "See remaining agents",
                )),
        }
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
