use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ExperienceCommands {
    Create {
        agent: String,
        sensation: String,
        description: String,
    },
    Show {
        id: String,
    },
    List {
        #[arg(long)]
        agent: Option<String>,
    },
    Update {
        id: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        sensation: Option<String>,
    },
}

impl ExperienceCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Response<Responses>, ExperienceError> {
        let client = context.client();
        let experience_client = ExperienceClient::new(&client);

        match self {
            ExperienceCommands::Create {
                agent,
                sensation,
                description,
            } => {
                let response = experience_client
                    .create(
                        AgentName::new(agent),
                        SensationName::new(sensation),
                        Description::new(description),
                    )
                    .await?;
                let ref_token = match &response {
                    ExperienceResponse::ExperienceCreated(e) => {
                        Some(RefToken::new(Ref::experience(e.id)))
                    }
                    _ => None,
                };
                let mut envelope = Response::new(response.into());
                if let Some(rt) = ref_token {
                    envelope = envelope.with_ref_token(rt);
                }
                Ok(envelope)
            }
            ExperienceCommands::Show { id } => {
                let id: ExperienceId = id.parse()?;
                Ok(Response::new(experience_client.get(&id).await?.into()))
            }
            ExperienceCommands::List { agent } => {
                let agent = agent.as_deref().map(AgentName::new);
                Ok(Response::new(
                    experience_client.list(agent.as_ref()).await?.into(),
                ))
            }
            ExperienceCommands::Update {
                id,
                description,
                sensation,
            } => {
                let id: ExperienceId = id.parse()?;
                let mut result: Option<ExperienceResponse> = None;
                if let Some(desc) = description {
                    result = Some(
                        experience_client
                            .update_description(&id, Description::new(desc))
                            .await?,
                    );
                }
                if let Some(sens) = sensation {
                    result = Some(
                        experience_client
                            .update_sensation(&id, SensationName::new(sens))
                            .await?,
                    );
                }
                match result {
                    Some(r) => Ok(Response::new(r.into())),
                    None => Err(ExperienceError::InvalidRequest(
                        "update requires --description or --sensation".into(),
                    )),
                }
            }
        }
    }
}
