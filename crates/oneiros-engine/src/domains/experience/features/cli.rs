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
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            ExperienceCommands::Create {
                agent,
                sensation,
                description,
            } => serde_json::to_string_pretty(&ExperienceService::create(
                ctx,
                agent,
                sensation,
                description,
            )?)?,
            ExperienceCommands::Show { id } => {
                serde_json::to_string_pretty(&ExperienceService::get(ctx, &id)?)?
            }
            ExperienceCommands::List { agent } => {
                serde_json::to_string_pretty(&ExperienceService::list(ctx, agent.as_deref())?)?
            }
            ExperienceCommands::Update {
                id,
                description,
                sensation,
            } => {
                let mut result = None;
                if let Some(desc) = description {
                    result = Some(ExperienceService::update_description(ctx, &id, desc)?);
                }
                if let Some(sens) = sensation {
                    result = Some(ExperienceService::update_sensation(ctx, &id, sens)?);
                }
                match result {
                    Some(r) => serde_json::to_string_pretty(&r)?,
                    None => return Err("update requires --description or --sensation".into()),
                }
            }
        };
        Ok(result)
    }
}
