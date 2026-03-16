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
    ) -> Result<(Outcomes<ShowTextureOutcomes>, Vec<PressureSummary>), TextureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client
            .get_texture(&context.ticket_token()?, &self.name)
            .await?;
        let summaries = response.pressure_summaries();
        let info: Texture = response.data()?;
        outcomes.emit(ShowTextureOutcomes::TextureDetails(info));

        Ok((outcomes, summaries))
    }
}
