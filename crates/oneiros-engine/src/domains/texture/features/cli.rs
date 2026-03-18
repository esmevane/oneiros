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
    Get {
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
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            TextureCommands::Set {
                name,
                description,
                prompt,
            } => serde_json::to_string_pretty(&TextureService::set(
                ctx,
                Texture {
                    name: TextureName::new(name),
                    description,
                    prompt,
                },
            )?)?,
            TextureCommands::Get { name } => {
                serde_json::to_string_pretty(&TextureService::get(ctx, &name)?)?
            }
            TextureCommands::List => serde_json::to_string_pretty(&TextureService::list(ctx)?)?,
            TextureCommands::Remove { name } => {
                serde_json::to_string_pretty(&TextureService::remove(ctx, &name)?)?
            }
        };
        Ok(result)
    }
}
