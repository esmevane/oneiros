use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum TextureCommands {
    Set(SetTexture),
    Show(GetTexture),
    List,
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
            TextureCommands::List => texture_client.list().await?,
            TextureCommands::Remove(removal) => texture_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            TextureResponse::TextureSet(name) => format!("Texture '{name}' set."),
            TextureResponse::TextureDetails(t) => {
                format!(
                    "Texture '{}'\n  Description: {}\n  Prompt: {}",
                    t.name, t.description, t.prompt
                )
            }
            TextureResponse::Textures(textures) => format!("Textures: {textures:?}"),
            TextureResponse::NoTextures => "No textures configured.".to_string(),
            TextureResponse::TextureRemoved(name) => format!("Texture '{name}' removed."),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
