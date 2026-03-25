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
    ) -> Result<Rendered<Responses>, ExperienceError> {
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
                let prompt = ref_token
                    .as_ref()
                    .map(|rt| format!("Experience recorded: {rt}"))
                    .unwrap_or_default();
                let mut envelope = Response::new(response.into());
                if let Some(rt) = ref_token {
                    envelope = envelope.with_ref_token(rt);
                }
                Ok(Rendered::new(envelope, prompt, String::new()))
            }
            ExperienceCommands::Show { id } => {
                let id: ExperienceId = id.parse()?;
                let response = experience_client.get(&id).await?;
                let prompt = match &response {
                    ExperienceResponse::ExperienceDetails(e) => {
                        format!("[{}] {}", e.sensation, e.description)
                    }
                    other => format!("{other:?}"),
                };
                Ok(Rendered::new(
                    Response::new(response.into()),
                    prompt,
                    String::new(),
                ))
            }
            ExperienceCommands::List { agent } => {
                let agent = agent.as_deref().map(AgentName::new);
                let response = experience_client.list(agent.as_ref()).await?;
                let prompt = match &response {
                    ExperienceResponse::Experiences(list) => format!("{} experiences.", list.len()),
                    ExperienceResponse::NoExperiences => "No experiences.".to_string(),
                    other => format!("{other:?}"),
                };
                Ok(Rendered::new(
                    Response::new(response.into()),
                    prompt,
                    String::new(),
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
                    Some(r) => {
                        let prompt = match &r {
                            ExperienceResponse::ExperienceUpdated(e) => {
                                format!(
                                    "Experience updated: {}",
                                    RefToken::new(Ref::experience(e.id))
                                )
                            }
                            other => format!("{other:?}"),
                        };
                        Ok(Rendered::new(
                            Response::new(r.into()),
                            prompt,
                            String::new(),
                        ))
                    }
                    None => Err(ExperienceError::InvalidRequest(
                        "update requires --description or --sensation".into(),
                    )),
                }
            }
        }
    }
}
