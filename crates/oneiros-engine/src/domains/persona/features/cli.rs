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
    Show {
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
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            PersonaCommands::Set {
                name,
                description,
                prompt,
            } => PersonaService::set(
                ctx,
                Persona {
                    name: PersonaName::new(name),
                    description,
                    prompt,
                },
            )?
            .into(),
            PersonaCommands::Show { name } => PersonaService::get(ctx, &name)?.into(),
            PersonaCommands::List => PersonaService::list(ctx)?.into(),
            PersonaCommands::Remove { name } => PersonaService::remove(ctx, &name)?.into(),
        };
        Ok(result)
    }
}
