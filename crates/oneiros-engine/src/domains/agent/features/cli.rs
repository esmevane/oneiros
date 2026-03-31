use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    Create(CreateAgent),
    Show(GetAgent),
    List,
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
            Self::List => agent_client.list().await?,
            Self::Update(update) => agent_client.update(update).await?,
            Self::Remove(removal) => agent_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            AgentResponse::AgentCreated(name) => format!("Agent '{name}' created."),
            AgentResponse::AgentDetails(a) => {
                format!(
                    "Agent '{}' (persona: {})\n  Description: {}\n  Prompt: {}",
                    a.name, a.persona, a.description, a.prompt
                )
            }
            AgentResponse::Agents(agents) => format!("Agents: {agents:?}"),
            AgentResponse::NoAgents => "No agents configured.".to_string(),
            AgentResponse::AgentUpdated(name) => format!("Agent '{name}' updated."),
            AgentResponse::AgentRemoved(name) => format!("Agent '{name}' removed."),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
