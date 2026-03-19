use clap::Subcommand;

use crate::*;

pub struct ExperienceCli;

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

impl ExperienceCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: ExperienceCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            ExperienceCommands::Create {
                agent,
                sensation,
                description,
            } => ExperienceService::create(ctx, agent, sensation, description)?.into(),
            ExperienceCommands::Show { id } => ExperienceService::get(ctx, &id)?.into(),
            ExperienceCommands::List { agent } => {
                ExperienceService::list(ctx, agent.as_deref())?.into()
            }
            ExperienceCommands::Update {
                id,
                description,
                sensation,
            } => {
                let mut result: Option<ExperienceResponse> = None;
                if let Some(desc) = description {
                    result = Some(ExperienceService::update_description(ctx, &id, desc)?);
                }
                if let Some(sens) = sensation {
                    result = Some(ExperienceService::update_sensation(ctx, &id, sens)?);
                }
                match result {
                    Some(r) => r.into(),
                    None => return Err("update requires --description or --sensation".into()),
                }
            }
        };
        Ok(result)
    }
}
