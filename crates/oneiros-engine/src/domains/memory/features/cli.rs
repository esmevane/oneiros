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
            Self::Add(addition) => (
                memory_client.add(addition).await?,
                MemoryRequest::AddMemory(addition.clone()),
            ),
            Self::Show(lookup) => (
                memory_client.get(lookup).await?,
                MemoryRequest::GetMemory(lookup.clone()),
            ),
            Self::List(listing) => (
                memory_client.list(listing).await?,
                MemoryRequest::ListMemories(listing.clone()),
            ),
        };

        Ok(MemoryView::new(response, &request).render().map(Into::into))
    }
}
