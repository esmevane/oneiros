use clap::Subcommand;

use crate::*;

pub struct PersonaCli;

#[derive(Debug, Subcommand)]
pub enum PersonaCommands {
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

impl PersonaCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: PersonaCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            PersonaCommands::Set {
                name,
                description,
                prompt,
            } => serde_json::to_string_pretty(&PersonaService::set(
                ctx,
                Persona {
                    name: PersonaName::new(name),
                    description,
                    prompt,
                },
            )?)?,
            PersonaCommands::Get { name } => {
                serde_json::to_string_pretty(&PersonaService::get(ctx, &name)?)?
            }
            PersonaCommands::List => {
                serde_json::to_string_pretty(&PersonaService::list(ctx)?)?
            }
            PersonaCommands::Remove { name } => {
                serde_json::to_string_pretty(&PersonaService::remove(ctx, &name)?)?
            }
        };
        Ok(result)
    }
}
