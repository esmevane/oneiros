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
    pub fn execute(&self, context: &ProjectContext) -> Result<Response<Responses>, MemoryError> {
        match self {
            MemoryCommands::Add {
                agent,
                level,
                content,
            } => {
                let response = MemoryService::add(
                    context,
                    &AgentName::new(&agent),
                    LevelName::new(&level),
                    Content::new(content),
                )?;
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
                Ok(Response::new(MemoryService::get(context, &id)?.into()))
            }
            MemoryCommands::List { agent } => Ok(Response::new(
                MemoryService::list(context, agent.as_deref().map(AgentName::new).as_ref())?.into(),
            )),
        }
    }
}
