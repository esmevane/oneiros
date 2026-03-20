use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum UrgeCommands {
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

impl UrgeCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, UrgeError> {
        let result = match self {
            UrgeCommands::Set {
                name,
                description,
                prompt,
            } => UrgeService::set(
                context,
                Urge::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )?
            .into(),
            UrgeCommands::Show { name } => UrgeService::get(context, &UrgeName::new(name))?.into(),
            UrgeCommands::List => UrgeService::list(context)?.into(),
            UrgeCommands::Remove { name } => {
                UrgeService::remove(context, &UrgeName::new(name))?.into()
            }
        };
        Ok(result)
    }
}
