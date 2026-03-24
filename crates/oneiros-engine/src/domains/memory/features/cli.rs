use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum MemoryCommands {
    Add {
        agent: String,
        level: String,
        content: String,
    },
    Show {
        id: String,
    },
    List {
        #[arg(long)]
        agent: Option<String>,
    },
}

impl MemoryCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, MemoryError> {
        let client = context.client();
        let memory_client = MemoryClient::new(&client);

        match self {
            MemoryCommands::Add {
                agent,
                level,
                content,
            } => {
                let response = memory_client
                    .add(
                        AgentName::new(agent),
                        LevelName::new(level),
                        Content::new(content),
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
            MemoryCommands::Show { id } => {
                let id: MemoryId = id.parse()?;
                let response = memory_client.get(&id).await?;
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
            MemoryCommands::List { agent } => {
                let agent = agent.as_deref().map(AgentName::new);
                let response = memory_client.list(agent.as_ref()).await?;
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
