use clap::Subcommand;

use crate::*;

pub struct SensationCli;

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

impl SensationCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: SensationCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            SensationCommands::Set {
                name,
                description,
                prompt,
            } => SensationService::set(
                ctx,
                Sensation {
                    name: SensationName::new(name),
                    description,
                    prompt,
                },
            )?
            .into(),
            SensationCommands::Show { name } => SensationService::get(ctx, &name)?.into(),
            SensationCommands::List => SensationService::list(ctx)?.into(),
            SensationCommands::Remove { name } => SensationService::remove(ctx, &name)?.into(),
        };
        Ok(result)
    }
}
