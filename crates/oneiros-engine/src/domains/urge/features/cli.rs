use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum UrgeCommands {
    Set(SetUrge),
    Show(GetUrge),
    List(ListUrges),
    Remove(RemoveUrge),
}

impl UrgeCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, UrgeError> {
        let client = context.client();
        let urge_client = UrgeClient::new(&client);

        let response = match self {
            UrgeCommands::Set(set) => urge_client.set(set).await?,
            UrgeCommands::Show(get) => urge_client.get(&get.name).await?,
            UrgeCommands::List(list) => urge_client.list(list).await?,
            UrgeCommands::Remove(removal) => urge_client.remove(&removal.name).await?,
        };

        Ok(UrgeView::new(response).render().map(Into::into))
    }
}
