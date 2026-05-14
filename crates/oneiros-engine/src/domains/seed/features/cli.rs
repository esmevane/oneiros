use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum SeedCommands {
    Core,
    Agents,
}

impl SeedCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, SeedError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            SeedCommands::Core => client.post("/seed/core", &()).await?,
            SeedCommands::Agents => client.post("/seed/agents", &()).await?,
        };

        let response: SeedResponse = serde_json::from_slice(&bytes)?;
        Ok(SeedView::new(response).render().map(Into::into))
    }
}
