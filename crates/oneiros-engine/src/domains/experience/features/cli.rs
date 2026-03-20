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
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Response<Responses>, ExperienceError> {
        match self {
            ExperienceCommands::Create {
                agent,
                sensation,
                description,
            } => {
                let response = ExperienceService::create(
                    context,
                    &AgentName::new(&agent),
                    SensationName::new(&sensation),
                    Description::new(&description),
                )?;
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
                Ok(Response::new(ExperienceService::get(context, &id)?.into()))
            }
            ExperienceCommands::List { agent } => Ok(Response::new(
                ExperienceService::list(context, agent.as_deref().map(AgentName::new).as_ref())?
                    .into(),
            )),
            ExperienceCommands::Update {
                id,
                description,
                sensation,
            } => {
                let id: ExperienceId = id.parse()?;
                let mut result: Option<ExperienceResponse> = None;
                if let Some(desc) = description {
                    result = Some(ExperienceService::update_description(
                        context,
                        &id,
                        Description::new(&desc),
                    )?);
                }
                if let Some(sens) = sensation {
                    result = Some(ExperienceService::update_sensation(
                        context,
                        &id,
                        SensationName::new(&sens),
                    )?);
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
