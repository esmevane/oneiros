mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::RemoveTextureOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RemoveTexture {
    /// The texture name to remove.
    name: TextureName,
}

impl RemoveTexture {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<RemoveTextureOutcomes>, TextureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        client
            .remove_texture(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveTextureOutcomes::TextureRemoved(self.name.clone()));

        Ok(outcomes)
    }
}
