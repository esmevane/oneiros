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
        let client = context.client();
        let seed = SeedClient::new(&client);
        let response = match self {
            SeedCommands::Core => seed.core().await?,
            SeedCommands::Agents => seed.agents().await?,
        };

        let prompt = match &response {
            SeedResponse::SeedComplete => SeedView::core_complete(),
            SeedResponse::AgentsSeedComplete => SeedView::agents_complete(),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
