use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum BrainCommands {
    Create(CreateBrain),
    Get(GetBrain),
    List(ListBrains),
}

impl BrainCommands {
    pub async fn execute(
        &self,
        context: &SystemContext,
    ) -> Result<Rendered<Responses>, BrainError> {
        let client = context.client();
        let brain_client = BrainClient::new(&client);

        let response = match self {
            BrainCommands::Create(create) => brain_client.create(create).await?,
            BrainCommands::Get(get) => brain_client.get(get).await?,
            BrainCommands::List(list) => brain_client.list(list).await?,
        };

        Ok(BrainView::new(response).render().map(Into::into))
    }
}
