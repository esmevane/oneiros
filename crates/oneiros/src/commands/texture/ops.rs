use clap::{Args, Subcommand};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum TextureCommandError {
    #[error("Client error: {0}")]
    Client(#[from] oneiros_client::Error),

    #[error(transparent)]
    Context(#[from] ContextError),
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum TextureOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetTextureOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveTextureOutcomes),
    #[outcome(transparent)]
    List(#[from] ListTexturesOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowTextureOutcomes),
}

#[derive(Clone, Args)]
pub struct TextureOps {
    #[command(subcommand)]
    pub command: TextureCommands,
}

impl TextureOps {
    pub async fn run(
        &self,
        context: &crate::Context,
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
pub enum TextureCommands {
    /// Create or update a texture.
    Set(SetTexture),
    /// Remove a texture.
    Remove(RemoveTexture),
    /// List all textures.
    List(ListTextures),
    /// Show a texture's details.
    Show(ShowTexture),
}
