use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum SeedCommands {
    Core,
    Agents,
}

impl SeedCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, SeedError> {
        let client = Client::new(config.base_url());
        let seed = SeedClient::new(&client);
        let response = match self {
            SeedCommands::Core => seed.core().await?,
            SeedCommands::Agents => seed.agents().await?,
        };

        Ok(SeedView::new(response).render().map(Into::into))
    }
}
