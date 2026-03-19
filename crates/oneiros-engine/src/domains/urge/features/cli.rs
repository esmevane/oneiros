use clap::Subcommand;

use crate::*;

pub struct UrgeCli;

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

impl UrgeCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: UrgeCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            UrgeCommands::Set {
                name,
                description,
                prompt,
            } => UrgeService::set(
                ctx,
                Urge {
                    name: UrgeName::new(name),
                    description: Description(description),
                    prompt: Prompt(prompt),
                },
            )?
            .into(),
            UrgeCommands::Show { name } => UrgeService::get(ctx, &name)?.into(),
            UrgeCommands::List => UrgeService::list(ctx)?.into(),
            UrgeCommands::Remove { name } => UrgeService::remove(ctx, &name)?.into(),
        };
        Ok(result)
    }
}
