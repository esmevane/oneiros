use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum LifecycleCommands {
    Wake { agent: String },
    Dream { agent: String },
    Introspect { agent: String },
    Reflect { agent: String },
    Sense { agent: String, content: String },
    Sleep { agent: String },
    Guidebook { agent: String },
}

impl LifecycleCommands {
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, LifecycleError> {
        let client = context.client();
        let lifecycle_client = LifecycleClient::new(&client);

        let result = match self {
            LifecycleCommands::Wake { agent } => {
                lifecycle_client.wake(&AgentName::new(agent)).await?.into()
            }
            LifecycleCommands::Dream { agent } => {
                lifecycle_client.dream(&AgentName::new(agent)).await?.into()
            }
            LifecycleCommands::Introspect { agent } => lifecycle_client
                .introspect(&AgentName::new(agent))
                .await?
                .into(),
            LifecycleCommands::Reflect { agent } => lifecycle_client
                .reflect(&AgentName::new(agent))
                .await?
                .into(),
            LifecycleCommands::Sense { agent, content } => lifecycle_client
                .sense(&AgentName::new(agent), Content::new(content))
                .await?
                .into(),
            LifecycleCommands::Sleep { agent } => {
                lifecycle_client.sleep(&AgentName::new(agent)).await?.into()
            }
            LifecycleCommands::Guidebook { agent } => lifecycle_client
                .guidebook(&AgentName::new(agent))
                .await?
                .into(),
        };
        Ok(result)
    }
}
