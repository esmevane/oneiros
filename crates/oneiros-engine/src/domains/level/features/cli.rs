use clap::Subcommand;

use crate::*;

pub struct LevelCli;

#[derive(Debug, Subcommand)]
pub enum LevelCommands {
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

impl LevelCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: LevelCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            LevelCommands::Set {
                name,
                description,
                prompt,
            } => serde_json::to_string_pretty(&LevelService::set(
                ctx,
                Level {
                    name: LevelName::new(name),
                    description,
                    prompt,
                },
            )?)?,
            LevelCommands::Get { name } => {
                serde_json::to_string_pretty(&LevelService::get(ctx, &name)?)?
            }
            LevelCommands::List => {
                serde_json::to_string_pretty(&LevelService::list(ctx)?)?
            }
            LevelCommands::Remove { name } => {
                serde_json::to_string_pretty(&LevelService::remove(ctx, &name)?)?
            }
        };
        Ok(result)
    }
}
