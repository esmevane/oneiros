use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum AgentCommands {
    Create {
        name: AgentName,
        persona: PersonaName,
        #[arg(long, default_value = "")]
        description: Description,
        #[arg(long, default_value = "")]
        prompt: Prompt,
    },
    Show {
        name: AgentName,
    },
    List,
    Update {
        name: AgentName,
        persona: PersonaName,
        #[arg(long, default_value = "")]
        description: Description,
        #[arg(long, default_value = "")]
        prompt: Prompt,
    },
    Remove {
        name: AgentName,
    },
}

impl AgentCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, AgentError> {
        let client = context.client();
        let agent_client = AgentClient::new(&client);

        let response = match self {
            Self::Create {
                name,
                persona,
                description,
                prompt,
            } => {
                agent_client
                    .create(
                        name.clone(),
                        persona.clone(),
                        description.clone(),
                        prompt.clone(),
                    )
                    .await?
            }
            Self::Show { name } => agent_client.get(name).await?,
            Self::List => agent_client.list().await?,
            Self::Update {
                name,
                persona,
                description,
                prompt,
            } => {
                agent_client
                    .update(name, persona.clone(), description.clone(), prompt.clone())
                    .await?
            }
            Self::Remove { name } => agent_client.remove(name).await?,
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
