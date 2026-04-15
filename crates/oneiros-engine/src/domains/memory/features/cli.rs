use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum MemoryCommands {
    Add(AddMemory),
    Show(GetMemory),
    List(ListMemories),
}

impl MemoryCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, MemoryError> {
        let client = context.client();
        let memory_client = MemoryClient::new(&client);

        let response = match self {
            MemoryCommands::Add(addition) => memory_client.add(addition).await?,
            MemoryCommands::Show(get) => memory_client.get(get).await?,
            MemoryCommands::List(listing) => memory_client.list(listing).await?,
        };

        Ok(MemoryView::new(response, self).render().map(Into::into))
    }
}
