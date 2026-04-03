use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SeedCommands {
    Core,
    Agents,
}

impl SeedCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, SeedError> {
        let response = match self {
            SeedCommands::Core => SeedService::core(context).await?,
            SeedCommands::Agents => SeedService::agents(context).await?,
        };

        let prompt = match &response {
            SeedResponse::SeedComplete => "Core seed complete.".to_string(),
            SeedResponse::AgentsSeedComplete => "Agent seed complete.".to_string(),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
