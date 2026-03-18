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
    Get {
        id: String,
    },
    List {
        #[arg(long)]
        agent: Option<String>,
    },
    UpdateDescription {
        id: String,
        description: String,
    },
    UpdateSensation {
        id: String,
        sensation: String,
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
            ExperienceCommands::Get { id } => {
                serde_json::to_string_pretty(&ExperienceService::get(ctx, &id)?)?
            }
            ExperienceCommands::List { agent } => {
                serde_json::to_string_pretty(&ExperienceService::list(ctx, agent.as_deref())?)?
            }
            ExperienceCommands::UpdateDescription { id, description } => {
                serde_json::to_string_pretty(&ExperienceService::update_description(
                    ctx,
                    &id,
                    description,
                )?)?
            }
            ExperienceCommands::UpdateSensation { id, sensation } => {
                serde_json::to_string_pretty(&ExperienceService::update_sensation(
                    ctx, &id, sensation,
                )?)?
            }
        };
        Ok(result)
    }
}
