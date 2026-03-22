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
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, TextureError> {
        let client = context.client();
        let texture_client = TextureClient::new(&client);

        let result = match self {
            TextureCommands::Set(texture) => texture_client.set(texture).await?.into(),
            TextureCommands::Show { name } => texture_client.get(name).await?.into(),
            TextureCommands::List => texture_client.list().await?.into(),
            TextureCommands::Remove { name } => texture_client.remove(name).await?.into(),
        };

        Ok(result)
    }
}
