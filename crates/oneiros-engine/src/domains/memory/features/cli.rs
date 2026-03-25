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

        match self {
            MemoryCommands::Add(addition) => {
                let response = memory_client
                    .add(
                        addition.agent.clone(),
                        addition.level.clone(),
                        addition.content.clone(),
                    )
                    .await?;
                let ref_token = match &response {
                    MemoryResponse::MemoryAdded(m) => Some(RefToken::new(Ref::memory(m.id))),
                    _ => None,
                };
                let prompt = ref_token
                    .as_ref()
                    .map(|rt| format!("Memory recorded: {rt}"))
                    .unwrap_or_default();
                let mut envelope = Response::new(response.into());
                if let Some(rt) = ref_token {
                    envelope = envelope.with_ref_token(rt);
                }
                Ok(Rendered::new(envelope, prompt, String::new()))
            }
            MemoryCommands::Show(get) => {
                let response = memory_client.get(&get.id).await?;
                let prompt = match &response {
                    MemoryResponse::MemoryDetails(m) => format!("[{}] {}", m.level, m.content),
                    other => format!("{other:?}"),
                };
                Ok(Rendered::new(
                    Response::new(response.into()),
                    prompt,
                    String::new(),
                ))
            }
            MemoryCommands::List(listing) => {
                let response = memory_client.list(listing.agent.as_ref()).await?;
                let prompt = match &response {
                    MemoryResponse::Memories(list) => format!("{} memories.", list.len()),
                    MemoryResponse::NoMemories => "No memories.".to_string(),
                    other => format!("{other:?}"),
                };
                Ok(Rendered::new(
                    Response::new(response.into()),
                    prompt,
                    String::new(),
                ))
            }
        }
    }
}
