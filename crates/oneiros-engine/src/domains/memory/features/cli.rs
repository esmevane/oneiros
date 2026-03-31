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
            MemoryResponse::MemoryAdded(m) => {
                format!("Memory recorded: {}", RefToken::new(Ref::memory(m.id)))
            }
            MemoryResponse::MemoryDetails(m) => format!("[{}] {}", m.level, m.content),
            MemoryResponse::Memories(list) => format!("{} memories.", list.len()),
            MemoryResponse::NoMemories => "No memories.".to_string(),
        };

        let envelope = match response.clone() {
            MemoryResponse::MemoryAdded(m) => {
                Response::new(response.into()).with_ref_token(RefToken::new(Ref::memory(m.id)))
            }
            otherwise => Response::new(otherwise.into()),
        };

        Ok(Rendered::new(envelope, prompt, String::new()))
    }
}
