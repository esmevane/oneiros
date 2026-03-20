use clap::Subcommand;

use crate::*;

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

impl PersonaCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, PersonaError> {
        let result = match self {
            PersonaCommands::Set {
                name,
                description,
                prompt,
            } => PersonaService::set(
                context,
                Persona::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )?
            .into(),
            PersonaCommands::Show { name } => {
                PersonaService::get(context, &PersonaName::new(name))?.into()
            }
            PersonaCommands::List => PersonaService::list(context)?.into(),
            PersonaCommands::Remove { name } => {
                PersonaService::remove(context, &PersonaName::new(name))?.into()
            }
        };
        Ok(result)
    }
}
