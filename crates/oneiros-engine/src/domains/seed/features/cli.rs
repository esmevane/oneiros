use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SeedCommands {
    Core,
    Agents,
}

impl SeedCommands {
    pub async fn execute(&self, context: &ProjectLog) -> Result<Rendered<Responses>, SeedError> {
        let client = context.client();
        let seed = SeedClient::new(&client);
        let response = match self {
            SeedCommands::Core => seed.core().await?,
            SeedCommands::Agents => seed.agents().await?,
        };

        Ok(SeedView::new(response).render().map(Into::into))
    }
}
