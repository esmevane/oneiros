use clap::Args;
use oneiros_client::Client;
use oneiros_model::TextureRecord;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListTexturesOutcomes {
    #[outcome(message("No textures configured."))]
    NoTextures,

    #[outcome(message("Textures: {0:?}"))]
    Textures(Vec<TextureRecord>),
}

#[derive(Clone, Args)]
pub struct ListTextures;

impl ListTextures {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListTexturesOutcomes>, TextureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let textures = client.list_textures(&context.ticket_token()?).await?;

        if textures.is_empty() {
            outcomes.emit(ListTexturesOutcomes::NoTextures);
        } else {
            outcomes.emit(ListTexturesOutcomes::Textures(textures));
        }

        Ok(outcomes)
    }
}
