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
    ) -> Result<Rendered<Responses>, CognitionError> {
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
                let prompt = ref_token
                    .as_ref()
                    .map(|rt| format!("Cognition recorded: {rt}"))
                    .unwrap_or_default();
                let mut envelope = Response::new(response.into());
                if let Some(rt) = ref_token {
                    envelope = envelope.with_ref_token(rt);
                }
                Ok(Rendered::new(envelope, prompt, String::new()))
            }
            CognitionCommands::Show { id } => {
                let id: CognitionId = id.parse()?;
                let response = cognition_client.get(&id).await?;
                let prompt = match &response {
                    CognitionResponse::CognitionDetails(c) => {
                        format!("[{}] {}", c.texture, c.content)
                    }
                    other => format!("{other:?}"),
                };
                Ok(Rendered::new(
                    Response::new(response.into()),
                    prompt,
                    String::new(),
                ))
            }
            CognitionCommands::List { agent, texture } => {
                let agent = agent.as_deref().map(AgentName::new);
                let texture = texture.as_deref().map(TextureName::new);
                let response = cognition_client
                    .list(agent.as_ref(), texture.as_ref())
                    .await?;
                let prompt = match &response {
                    CognitionResponse::Cognitions(list) => format!("{} cognitions.", list.len()),
                    CognitionResponse::NoCognitions => "No cognitions.".to_string(),
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
