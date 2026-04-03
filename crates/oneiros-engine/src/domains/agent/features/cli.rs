use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    Create(CreateAgent),
    Show(GetAgent),
    List(ListAgents),
    Update(UpdateAgent),
    Remove(RemoveAgent),
}

impl AgentCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, AgentError> {
        let client = context.client();
        let agent_client = AgentClient::new(&client);

        let response = match self {
            Self::Create(creation) => agent_client.create(creation).await?,
            Self::Show(get) => agent_client.get(&get.name).await?,
            Self::List(listing) => agent_client.list(listing).await?,
            Self::Update(update) => agent_client.update(update).await?,
            Self::Remove(removal) => agent_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            AgentResponse::AgentCreated(name) => format!("Agent '{name}' created."),
            AgentResponse::AgentDetails(wrapped) => {
                format!(
                    "Agent '{}' (persona: {})\n  Description: {}\n  Prompt: {}",
                    wrapped.data.name,
                    wrapped.data.persona,
                    wrapped.data.description,
                    wrapped.data.prompt
                )
            }
            AgentResponse::Agents(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    out.push_str(&format!(
                        "  {} ({})\n    {}\n\n",
                        wrapped.data.name, wrapped.data.persona, wrapped.data.description
                    ));
                }
                out
            }
            AgentResponse::NoAgents => "No agents configured.".to_string(),
            AgentResponse::AgentUpdated(name) => format!("Agent '{name}' updated."),
            AgentResponse::AgentRemoved(name) => format!("Agent '{name}' removed."),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
