use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum NatureCommands {
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

impl NatureCommands {
    pub fn execute(&self, context: &ProjectContext) -> Result<Responses, NatureError> {
        let result = match self {
            NatureCommands::Set {
                name,
                description,
                prompt,
            } => NatureService::set(
                context,
                Nature::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )?
            .into(),
            NatureCommands::Show { name } => {
                NatureService::get(context, &NatureName::new(name))?.into()
            }
            NatureCommands::List => NatureService::list(context)?.into(),
            NatureCommands::Remove { name } => {
                NatureService::remove(context, &NatureName::new(name))?.into()
            }
        };
        Ok(result)
    }
}
