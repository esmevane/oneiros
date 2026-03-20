use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum LevelCommands {
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

impl LevelCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match self {
            LevelCommands::Set {
                name,
                description,
                prompt,
            } => LevelService::set(
                context,
                Level::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )?
            .into(),
            LevelCommands::Show { name } => {
                LevelService::get(context, &LevelName::new(name))?.into()
            }
            LevelCommands::List => LevelService::list(context)?.into(),
            LevelCommands::Remove { name } => {
                LevelService::remove(context, &LevelName::new(name))?.into()
            }
        };
        Ok(result)
    }
}
