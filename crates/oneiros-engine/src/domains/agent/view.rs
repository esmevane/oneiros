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
            AgentResponse::AgentCreated(AgentCreatedResponse::V1(created)) => {
                let agent = &created.agent;
                McpResponse::new(format!("Agent created: {}", agent.name)).hint_set(
                    HintSet::agent_created(
                        AgentCreatedHints::builder()
                            .agent(agent.name.clone())
                            .build(),
                    ),
                )
            }
            AgentResponse::AgentDetails(AgentDetailsResponse::V1(details)) => {
                let agent = &details.agent;
                McpResponse::new(format!(
                    "# {}\n\n**persona:** {}\n**description:** {}\n\n{}\n",
                    agent.name, agent.persona, agent.description, agent.prompt
                ))
                .hint(Hint::inspect(
                    ResourcePath::AgentCognitions(agent.name.clone()).uri(),
                    "Browse cognitions",
                ))
                .hint(Hint::inspect(
                    ResourcePath::AgentPressure(agent.name.clone()).uri(),
                    "Check pressure",
                ))
            }
            AgentResponse::Agents(AgentsResponse::V1(agents)) => {
                let mut md = format!(
                    "# Agents\n\n{} of {} total\n\n",
                    agents.items.len(),
                    agents.total
                );
                md.push_str("| Name | Persona | Description |\n");
                md.push_str("|------|---------|-------------|\n");
                for agent in &agents.items {
                    md.push_str(&format!(
                        "| {} | {} | {} |\n",
                        agent.name, agent.persona, agent.description
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
            AgentResponse::AgentUpdated(AgentUpdatedResponse::V1(updated)) => {
                let agent = &updated.agent;
                McpResponse::new(format!("Agent updated: {}", agent.name)).hint(Hint::inspect(
                    ResourcePath::Agent(agent.name.clone()).uri(),
                    "View agent details",
                ))
            }
            AgentResponse::AgentRemoved(AgentRemovedResponse::V1(removed)) => {
                McpResponse::new(format!("Agent removed: {}", removed.name)).hint(Hint::inspect(
                    ResourcePath::Agents.uri(),
                    "See remaining agents",
                ))
            }
        }
    }

    pub fn render(self) -> Rendered<AgentResponse> {
        match self.response {
            AgentResponse::AgentCreated(AgentCreatedResponse::V1(created)) => {
                let prompt = Confirmation::new("Agent", created.agent.name.to_string(), "created")
                    .to_string();
                let hints = HintSet::agent_created(
                    AgentCreatedHints::builder()
                        .agent(created.agent.name.clone())
                        .build(),
                );
                Rendered::new(
                    AgentResponse::AgentCreated(AgentCreatedResponse::V1(created)),
                    prompt,
                    String::new(),
                )
                .with_hints(hints)
            }
            AgentResponse::AgentDetails(AgentDetailsResponse::V1(details)) => {
                let prompt = Detail::new(details.agent.name.to_string())
                    .field("persona:", details.agent.persona.to_string())
                    .field("description:", details.agent.description.to_string())
                    .field("prompt:", details.agent.prompt.to_string())
                    .to_string();
                Rendered::new(
                    AgentResponse::AgentDetails(AgentDetailsResponse::V1(details)),
                    prompt,
                    String::new(),
                )
            }
            AgentResponse::Agents(AgentsResponse::V1(agents)) => {
                let mut table = Table::new(vec![
                    Column::key("name", "Name"),
                    Column::key("persona", "Persona"),
                    Column::key("description", "Description").max(60),
                ]);
                for agent in &agents.items {
                    table.push_row(vec![
                        agent.name.to_string(),
                        agent.persona.to_string(),
                        agent.description.to_string(),
                    ]);
                }
                let prompt = format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", agents.items.len(), agents.total).muted(),
                );
                Rendered::new(
                    AgentResponse::Agents(AgentsResponse::V1(agents)),
                    prompt,
                    String::new(),
                )
            }
            AgentResponse::NoAgents => Rendered::new(
                AgentResponse::NoAgents,
                format!("{}", "No agents configured.".muted()),
                String::new(),
            ),
            AgentResponse::AgentUpdated(AgentUpdatedResponse::V1(updated)) => {
                let prompt = Confirmation::new("Agent", updated.agent.name.to_string(), "updated")
                    .to_string();
                Rendered::new(
                    AgentResponse::AgentUpdated(AgentUpdatedResponse::V1(updated)),
                    prompt,
                    String::new(),
                )
            }
            AgentResponse::AgentRemoved(AgentRemovedResponse::V1(removed)) => {
                let prompt =
                    Confirmation::new("Agent", removed.name.to_string(), "removed").to_string();
                Rendered::new(
                    AgentResponse::AgentRemoved(AgentRemovedResponse::V1(removed)),
                    prompt,
                    String::new(),
                )
            }
        }
    }
}
