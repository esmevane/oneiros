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
    Show {
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
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            NatureCommands::Set {
                name,
                description,
                prompt,
            } => NatureService::set(
                ctx,
                Nature {
                    name: NatureName::new(name),
                    description: Description(description),
                    prompt: Prompt(prompt),
                },
            )?
            .into(),
            NatureCommands::Show { name } => NatureService::get(ctx, &name)?.into(),
            NatureCommands::List => NatureService::list(ctx)?.into(),
            NatureCommands::Remove { name } => NatureService::remove(ctx, &name)?.into(),
        };
        Ok(result)
    }
}
