mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListTexturesOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListTextures;

impl ListTextures {
    pub(crate) async fn run(
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
