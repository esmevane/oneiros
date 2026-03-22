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
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Response<Responses>, CognitionError> {
        let client = context.client();
        let cognition_client = CognitionClient::new(&client);

        match self {
            CognitionCommands::Add {
                agent,
                texture,
                content,
            } => {
                let response = cognition_client
                    .add(
                        AgentName::new(agent),
                        TextureName::new(texture),
                        Content::new(content),
                    )
                    .await?;
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
                Ok(Response::new(cognition_client.get(&id).await?.into()))
            }
            CognitionCommands::List { agent, texture } => {
                let agent = agent.as_deref().map(AgentName::new);
                let texture = texture.as_deref().map(TextureName::new);
                Ok(Response::new(
                    cognition_client
                        .list(agent.as_ref(), texture.as_ref())
                        .await?
                        .into(),
                ))
            }
        }
    }
}
