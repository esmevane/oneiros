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
    Show {
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
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            LevelCommands::Set {
                name,
                description,
                prompt,
            } => LevelService::set(
                ctx,
                Level {
                    name: LevelName::new(name),
                    description,
                    prompt,
                },
            )?
            .into(),
            LevelCommands::Show { name } => LevelService::get(ctx, &name)?.into(),
            LevelCommands::List => LevelService::list(ctx)?.into(),
            LevelCommands::Remove { name } => LevelService::remove(ctx, &name)?.into(),
        };
        Ok(result)
    }
}
