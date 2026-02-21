mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::SetTextureOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct SetTexture {
    /// The texture name (identity).
    pub(crate) name: TextureName,

    /// A human-readable description of the texture's purpose.
    #[arg(long, default_value = "")]
    pub(crate) description: Description,

    /// Guidance text for agents when logging cognition with this texture.
    #[arg(long, default_value = "")]
    pub(crate) prompt: Prompt,
}

impl SetTexture {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SetTextureOutcomes>, TextureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .set_texture(
                &context.ticket_token()?,
                TextureRecord::init(
                    self.description.clone(),
                    self.prompt.clone(),
                    Texture {
                        name: self.name.clone(),
                    },
                ),
            )
            .await?;
        outcomes.emit(SetTextureOutcomes::TextureSet(info.name.clone()));

        Ok(outcomes)
    }
}
