use clap::Args;
use oneiros_client::Client;
use oneiros_model::TextureRecord;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowTextureOutcomes {
    #[outcome(message("Texture '{}'\n  Description: {}\n  Prompt: {}", .0.name, .0.description, .0.prompt))]
    TextureDetails(TextureRecord),
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

        let client = Client::new(context.socket_path());

        let info = client
            .get_texture(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(ShowTextureOutcomes::TextureDetails(info));

        Ok(outcomes)
    }
}
