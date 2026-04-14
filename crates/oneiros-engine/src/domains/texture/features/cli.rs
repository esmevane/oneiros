use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum TextureCommands {
    Set(SetTexture),
    Show(GetTexture),
    List(ListTextures),
    Remove(RemoveTexture),
}

impl TextureCommands {
    pub(crate) async fn execute(
        &self,
        client: &Client,
    ) -> Result<Rendered<Responses>, TextureError> {
        
        let texture_client = TextureClient::new(client);

        let response = match self {
            TextureCommands::Set(set) => texture_client.set(set).await?,
            TextureCommands::Show(get) => texture_client.get(&get.name).await?,
            TextureCommands::List(list) => texture_client.list(list).await?,
            TextureCommands::Remove(removal) => texture_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            TextureResponse::TextureSet(name) => TextureView::confirmed("set", name).to_string(),
            TextureResponse::TextureDetails(wrapped) => {
                TextureView::detail(&wrapped.data).to_string()
            }
            TextureResponse::Textures(listed) => {
                let table = TextureView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            TextureResponse::NoTextures => format!("{}", "No textures configured.".muted()),
            TextureResponse::TextureRemoved(name) => {
                TextureView::confirmed("removed", name).to_string()
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
