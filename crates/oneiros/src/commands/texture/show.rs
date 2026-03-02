use clap::Args;
use oneiros_model::Texture;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowTextureOutcomes {
    #[outcome(message("Texture '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    TextureDetails(Texture),
}

#[derive(Clone, Args)]
pub struct ShowTexture {
    /// The texture name to display.
    name: TextureName,
}

impl ShowTexture {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowTextureOutcomes>, TextureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let info = client
            .get_texture(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(ShowTextureOutcomes::TextureDetails(info));

        Ok(outcomes)
    }
}
