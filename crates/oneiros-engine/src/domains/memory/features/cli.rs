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

        let prompt = match &response {
            MemoryResponse::MemoryAdded(wrapped) => MemoryView::recorded(wrapped),
            MemoryResponse::MemoryDetails(wrapped) => MemoryView::detail(&wrapped.data).to_string(),
            MemoryResponse::Memories(listed) => {
                let table = MemoryView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            MemoryResponse::NoMemories => format!("{}", "No memories.".muted()),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
