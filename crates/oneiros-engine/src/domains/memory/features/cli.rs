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
    ) -> Result<Response<Responses>, MemoryError> {
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
                let mut envelope = Response::new(response.into());
                if let Some(rt) = ref_token {
                    envelope = envelope.with_ref_token(rt);
                }
                Ok(envelope)
            }
            MemoryCommands::Show { id } => {
                let id: MemoryId = id.parse()?;
                Ok(Response::new(memory_client.get(&id).await?.into()))
            }
            MemoryCommands::List { agent } => {
                let agent = agent.as_deref().map(AgentName::new);
                Ok(Response::new(
                    memory_client.list(agent.as_ref()).await?.into(),
                ))
            }
        }
    }
}
