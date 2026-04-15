use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum TextureCommands {
    Set(SetTexture),
    Show(GetTexture),
    List(ListTextures),
    Remove(RemoveTexture),
}

impl TextureCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, TextureError> {
        let client = context.client();
        let texture_client = TextureClient::new(&client);

        let response = match self {
            TextureCommands::Set(set) => texture_client.set(set).await?,
            TextureCommands::Show(get) => texture_client.get(&get.name).await?,
            TextureCommands::List(list) => texture_client.list(list).await?,
            TextureCommands::Remove(removal) => texture_client.remove(&removal.name).await?,
        };

        Ok(TextureView::new(response).render().map(Into::into))
    }
}
