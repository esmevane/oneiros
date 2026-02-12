mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::SetTextureOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct SetTexture {
    /// The texture name (identity).
    name: TextureName,

    /// A human-readable description of the texture's purpose.
    #[arg(long, default_value = "")]
    description: Description,

    /// Guidance text for agents when logging cognition with this texture.
    #[arg(long, default_value = "")]
    prompt: Prompt,
}

impl SetTexture {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<SetTextureOutcomes>, TextureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .set_texture(
                &context.ticket_token()?,
                Texture {
                    name: self.name.clone(),
                    description: self.description.clone(),
                    prompt: self.prompt.clone(),
                },
            )
            .await?;
        outcomes.emit(SetTextureOutcomes::TextureSet(info.name));

        Ok(outcomes)
    }
}
