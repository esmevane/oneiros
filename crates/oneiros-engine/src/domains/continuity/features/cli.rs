use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ContinuityCommands {
    Wake { agent: AgentName },
    Dream { agent: AgentName },
    Introspect { agent: AgentName },
    Reflect { agent: AgentName },
    Sense { agent: AgentName, content: Content },
    Sleep { agent: AgentName },
    Guidebook { agent: AgentName },
}

impl ContinuityCommands {
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, ContinuityError> {
        let client = context.client();
        let continuity_client = ContinuityClient::new(&client);

        let result = match self {
            ContinuityCommands::Wake { agent } => continuity_client.wake(agent).await?.into(),
            ContinuityCommands::Dream { agent } => continuity_client.dream(agent).await?.into(),
            ContinuityCommands::Introspect { agent } => {
                continuity_client.introspect(agent).await?.into()
            }
            ContinuityCommands::Reflect { agent } => continuity_client.reflect(agent).await?.into(),
            ContinuityCommands::Sense { agent, content } => continuity_client
                .sense(agent, content.clone())
                .await?
                .into(),
            ContinuityCommands::Sleep { agent } => continuity_client.sleep(agent).await?.into(),
            ContinuityCommands::Guidebook { agent } => {
                continuity_client.guidebook(agent).await?.into()
            }
        };
        Ok(result)
    }
}
