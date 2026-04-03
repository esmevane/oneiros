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
            MemoryResponse::MemoryAdded(wrapped) => wrapped
                .meta()
                .ref_token()
                .map(|ref_token| format!("Memory recorded: {ref_token}"))
                .unwrap_or_default(),
            MemoryResponse::MemoryDetails(wrapped) => {
                format!("[{}] {}", wrapped.data.level, wrapped.data.content)
            }
            MemoryResponse::Memories(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    let ref_token = wrapped
                        .meta()
                        .ref_token()
                        .map(|ref_token| ref_token.to_string())
                        .unwrap_or_default();
                    out.push_str(&format!(
                        "  [{}] {}\n    {}\n\n",
                        wrapped.data.level, wrapped.data.content, ref_token
                    ));
                }
                out
            }
            MemoryResponse::NoMemories => "No memories.".to_string(),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
