use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum NatureCommands {
    Set(SetNature),
    Show(GetNature),
    List(ListNatures),
    Remove(RemoveNature),
}

impl NatureCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, NatureError> {
        let client = context.client();
        let nature_client = NatureClient::new(&client);

        let response = match self {
            NatureCommands::Set(set) => nature_client.set(set).await?,
            NatureCommands::Show(get) => nature_client.get(get).await?,
            NatureCommands::List(list) => nature_client.list(list).await?,
            NatureCommands::Remove(removal) => nature_client.remove(&removal.name).await?,
        };

        Ok(NatureView::new(response).render().map(Into::into))
    }
}
