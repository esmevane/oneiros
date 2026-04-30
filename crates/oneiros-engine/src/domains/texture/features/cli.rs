use clap::Subcommand;

use crate::*;

/// CLI subcommands for the texture domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct. The
/// dispatcher passes the wrapper through to the client without rebuilding,
/// since the operation type *is* the domain command.
#[derive(Debug, Subcommand)]
pub enum TextureCommands {
    Set(SetTexture),
    Show(GetTexture),
    List(ListTextures),
    Remove(RemoveTexture),
}

impl TextureCommands {
    pub async fn execute(&self, context: &ProjectLog) -> Result<Rendered<Responses>, TextureError> {
        let client = context.client();
        let texture_client = TextureClient::new(&client);

        let response = match self {
            Self::Set(setting) => texture_client.set(setting).await?,
            Self::Show(lookup) => texture_client.get(lookup).await?,
            Self::List(listing) => texture_client.list(listing).await?,
            Self::Remove(removal) => texture_client.remove(removal).await?,
        };

        Ok(TextureView::new(response).render().map(Into::into))
    }
}
