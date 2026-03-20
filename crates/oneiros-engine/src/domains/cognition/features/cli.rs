use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum CognitionCommands {
    Add {
        agent: String,
        texture: String,
        content: String,
    },
    Show {
        id: String,
    },
    List {
        #[arg(long)]
        agent: Option<String>,
        #[arg(long)]
        texture: Option<String>,
    },
}

impl CognitionCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Response<Responses>, CognitionError> {
        match self {
            CognitionCommands::Add {
                agent,
                texture,
                content,
            } => {
                let response = CognitionService::add(
                    context,
                    &AgentName::new(&agent),
                    TextureName::new(&texture),
                    Content::new(content),
                )?;
                let ref_token = match &response {
                    CognitionResponse::CognitionAdded(c) => {
                        Some(RefToken::new(Ref::cognition(c.id)))
                    }
                    _ => None,
                };
                let mut envelope = Response::new(response.into());
                if let Some(rt) = ref_token {
                    envelope = envelope.with_ref_token(rt);
                }
                Ok(envelope)
            }
            CognitionCommands::Show { id } => {
                let id: CognitionId = id.parse()?;
                Ok(Response::new(CognitionService::get(context, &id)?.into()))
            }
            CognitionCommands::List { agent, texture } => Ok(Response::new(
                CognitionService::list(
                    context,
                    agent.as_deref().map(AgentName::new).as_ref(),
                    texture.as_deref().map(TextureName::new).as_ref(),
                )?
                .into(),
            )),
        }
    }
}
