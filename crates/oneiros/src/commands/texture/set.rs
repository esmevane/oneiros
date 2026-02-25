use clap::Args;
use oneiros_client::Client;
use oneiros_model::TextureName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetTextureOutcomes {
    #[outcome(message("Texture '{0}' set."))]
    TextureSet(TextureName),
}

#[derive(Clone, Args)]
pub struct SetTexture {
    /// The texture name (identity).
    pub name: TextureName,

    /// A human-readable description of the texture's purpose.
    #[arg(long, default_value = "")]
    pub description: Description,

    /// Guidance text for agents when logging cognition with this texture.
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

impl SetTexture {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SetTextureOutcomes>, TextureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .set_texture(
                &context.ticket_token()?,
                Texture::init(
                    self.name.clone(),
                    self.description.clone(),
                    self.prompt.clone(),
                ),
            )
            .await?;
        outcomes.emit(SetTextureOutcomes::TextureSet(info.name.clone()));

        Ok(outcomes)
    }
}
