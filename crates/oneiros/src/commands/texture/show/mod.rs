mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowTextureOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowTexture {
    /// The texture name to display.
    name: TextureName,
}

impl ShowTexture {
    pub(crate) async fn run(
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
