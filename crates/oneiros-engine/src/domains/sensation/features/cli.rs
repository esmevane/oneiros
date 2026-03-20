use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SensationCommands {
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

impl SensationCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, SensationError> {
        let result = match self {
            SensationCommands::Set {
                name,
                description,
                prompt,
            } => SensationService::set(
                context,
                Sensation::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )?
            .into(),
            SensationCommands::Show { name } => {
                SensationService::get(context, &SensationName::new(name))?.into()
            }
            SensationCommands::List => SensationService::list(context)?.into(),
            SensationCommands::Remove { name } => {
                SensationService::remove(context, &SensationName::new(name))?.into()
            }
        };
        Ok(result)
    }
}
