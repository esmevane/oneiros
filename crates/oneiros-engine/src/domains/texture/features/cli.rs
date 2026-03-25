use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum TextureCommands {
    Set(Texture),
    Show { name: TextureName },
    List,
    Remove { name: TextureName },
}

impl TextureCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, TextureError> {
        let client = context.client();
        let texture_client = TextureClient::new(&client);

        let response = match self {
            TextureCommands::Set(texture) => texture_client.set(texture).await?,
            TextureCommands::Show { name } => texture_client.get(name).await?,
            TextureCommands::List => texture_client.list().await?,
            TextureCommands::Remove { name } => texture_client.remove(name).await?,
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
