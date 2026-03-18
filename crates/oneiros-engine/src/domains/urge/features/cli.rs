use clap::Subcommand;

use crate::*;

pub struct UrgeCli;

#[derive(Debug, Subcommand)]
pub enum UrgeCommands {
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

impl UrgeCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: UrgeCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            UrgeCommands::Set {
                name,
                description,
                prompt,
            } => serde_json::to_string_pretty(&UrgeService::set(
                ctx,
                Urge {
                    name: UrgeName::new(name),
                    description,
                    prompt,
                },
            )?)?,
            UrgeCommands::Get { name } => {
                serde_json::to_string_pretty(&UrgeService::get(ctx, &name)?)?
            }
            UrgeCommands::List => {
                serde_json::to_string_pretty(&UrgeService::list(ctx)?)?
            }
            UrgeCommands::Remove { name } => {
                serde_json::to_string_pretty(&UrgeService::remove(ctx, &name)?)?
            }
        };
        Ok(result)
    }
}
