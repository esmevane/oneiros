use clap::Subcommand;

use crate::*;

pub struct TextureCli;

#[derive(Debug, Subcommand)]
pub enum TextureCommands {
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

impl TextureCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: TextureCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            TextureCommands::Set {
                name,
                description,
                prompt,
            } => TextureService::set(
                ctx,
                Texture {
                    name: TextureName::new(name),
                    description,
                    prompt,
                },
            )?
            .into(),
            TextureCommands::Show { name } => TextureService::get(ctx, &name)?.into(),
            TextureCommands::List => TextureService::list(ctx)?.into(),
            TextureCommands::Remove { name } => TextureService::remove(ctx, &name)?.into(),
        };
        Ok(result)
    }
}
