use clap::Subcommand;

use crate::*;

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

impl TextureCommands {
    pub fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match self {
            TextureCommands::Set {
                name,
                description,
                prompt,
            } => TextureService::set(
                context,
                Texture::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )?
            .into(),
            TextureCommands::Show { name } => {
                TextureService::get(context, &TextureName::new(name))?.into()
            }
            TextureCommands::List => TextureService::list(context)?.into(),
            TextureCommands::Remove { name } => {
                TextureService::remove(context, &TextureName::new(name))?.into()
            }
        };
        Ok(result)
    }
}
