use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum SeedCommands {
    Core,
    Agents,
}

impl SeedCommands {
    pub(crate) async fn execute(
        &self,
        client: &Client,
    ) -> Result<Rendered<Responses>, SeedError> {
        let response = match self {
            SeedCommands::Core => SeedService::core(client).await?,
            SeedCommands::Agents => SeedService::agents(client).await?,
        };

        let prompt = match &response {
            SeedResponse::SeedComplete => SeedView::core_complete(),
            SeedResponse::AgentsSeedComplete => SeedView::agents_complete(),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
