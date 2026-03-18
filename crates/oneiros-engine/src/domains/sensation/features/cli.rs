use clap::Subcommand;

use crate::*;

pub struct SensationCli;

#[derive(Debug, Subcommand)]
pub enum SensationCommands {
    Set {
        name: String,
        #[arg(long, default_value = "")]
        description: String,
        #[arg(long, default_value = "")]
        prompt: String,
    },
    Get {
        name: String,
    },
    List,
    Remove {
        name: String,
    },
}

impl SensationCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: SensationCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            SensationCommands::Set {
                name,
                description,
                prompt,
            } => serde_json::to_string_pretty(&SensationService::set(
                ctx,
                Sensation {
                    name: SensationName::new(name),
                    description,
                    prompt,
                },
            )?)?,
            SensationCommands::Get { name } => {
                serde_json::to_string_pretty(&SensationService::get(ctx, &name)?)?
            }
            SensationCommands::List => {
                serde_json::to_string_pretty(&SensationService::list(ctx)?)?
            }
            SensationCommands::Remove { name } => {
                serde_json::to_string_pretty(&SensationService::remove(ctx, &name)?)?
            }
        };
        Ok(result)
    }
}
