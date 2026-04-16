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

        let (response, request) = match self {
            MemoryCommands::Add(addition) => {
                let response = memory_client.add(addition).await?;
                (response, MemoryRequest::AddMemory(addition.clone()))
            }
            MemoryCommands::Show(get) => {
                let response = memory_client.get(get).await?;
                (response, MemoryRequest::GetMemory(get.clone()))
            }
            MemoryCommands::List(listing) => {
                let response = memory_client.list(listing).await?;
                (response, MemoryRequest::ListMemories(listing.clone()))
            }
        };

        Ok(MemoryView::new(response, &request).render().map(Into::into))
    }
}
