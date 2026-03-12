use clap::Args;
use oneiros_model::TextureName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveTextureOutcomes {
    #[outcome(message("Texture '{0}' removed."))]
    TextureRemoved(TextureName),
}

#[derive(Clone, Args)]
pub struct RemoveTexture {
    /// The texture name to remove.
    name: TextureName,
}

impl RemoveTexture {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemoveTextureOutcomes>, TextureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        client
            .remove_texture(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveTextureOutcomes::TextureRemoved(self.name.clone()));

        Ok(outcomes)
    }
}
