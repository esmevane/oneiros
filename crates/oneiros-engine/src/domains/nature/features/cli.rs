use clap::Subcommand;

use crate::*;

pub struct NatureCli;

#[derive(Debug, Subcommand)]
pub enum NatureCommands {
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

impl NatureCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: NatureCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            NatureCommands::Set {
                name,
                description,
                prompt,
            } => serde_json::to_string_pretty(&NatureService::set(
                ctx,
                Nature {
                    name: NatureName::new(name),
                    description,
                    prompt,
                },
            )?)?,
            NatureCommands::Get { name } => {
                serde_json::to_string_pretty(&NatureService::get(ctx, &name)?)?
            }
            NatureCommands::List => {
                serde_json::to_string_pretty(&NatureService::list(ctx)?)?
            }
            NatureCommands::Remove { name } => {
                serde_json::to_string_pretty(&NatureService::remove(ctx, &name)?)?
            }
        };
        Ok(result)
    }
}
