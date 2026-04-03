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

        let prompt = match &response {
            TextureResponse::TextureSet(name) => format!("Texture '{name}' set."),
            TextureResponse::TextureDetails(t) => {
                format!(
                    "Texture '{}'\n  Description: {}\n  Prompt: {}",
                    t.name, t.description, t.prompt
                )
            }
            TextureResponse::Textures(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for texture in &listed.items {
                    out.push_str(&format!("  {} — {}\n\n", texture.name, texture.description,));
                }
                out
            }
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
