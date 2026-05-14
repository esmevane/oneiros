use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum MemoryCommands {
    Add(AddMemory),
    Show(GetMemory),
    List(ListMemories),
}

impl MemoryCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, MemoryError> {
        let client = Client::from_config(config)?;

        let (bytes, request) = match self {
            Self::Add(addition) => (
                addition.execute_request(&client).await?,
                MemoryRequest::AddMemory(addition.clone()),
            ),
            Self::Show(lookup) => (
                lookup.execute_request(&client).await?,
                MemoryRequest::GetMemory(lookup.clone()),
            ),
            Self::List(listing) => (
                listing.execute_request(&client).await?,
                MemoryRequest::ListMemories(listing.clone()),
            ),
        };

        let response: MemoryResponse = serde_json::from_slice(&bytes)?;
        Ok(MemoryView::new(response, &request).render().map(Into::into))
    }
}
