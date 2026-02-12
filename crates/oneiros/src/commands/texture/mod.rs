mod outcomes;

mod error;
mod list;
mod remove;
mod set;
mod show;

pub(crate) use error::TextureCommandError;
pub(crate) use list::{ListTextures, ListTexturesOutcomes};
pub(crate) use outcomes::TextureOutcomes;
pub(crate) use remove::{RemoveTexture, RemoveTextureOutcomes};
pub(crate) use set::{SetTexture, SetTextureOutcomes};
pub(crate) use show::{ShowTexture, ShowTextureOutcomes};

use clap::{Args, Subcommand};
use oneiros_outcomes::Outcomes;

#[derive(Clone, Args)]
pub(crate) struct TextureOps {
    #[command(subcommand)]
    pub command: TextureCommands,
}

impl TextureOps {
    pub(crate) async fn run(
        &self,
        context: crate::Context,
    ) -> Result<Outcomes<TextureOutcomes>, TextureCommandError> {
        Ok(match &self.command {
            TextureCommands::Set(cmd) => cmd.run(context).await?.map_into(),
            TextureCommands::Remove(cmd) => cmd.run(context).await?.map_into(),
            TextureCommands::List(cmd) => cmd.run(context).await?.map_into(),
            TextureCommands::Show(cmd) => cmd.run(context).await?.map_into(),
        })
    }
}

#[derive(Clone, Subcommand)]
pub(crate) enum TextureCommands {
    /// Create or update a texture.
    Set(SetTexture),
    /// Remove a texture.
    Remove(RemoveTexture),
    /// List all textures.
    List(ListTextures),
    /// Show a texture's details.
    Show(ShowTexture),
}
