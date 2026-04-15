use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum LevelCommands {
    Set(SetLevel),
    Show(GetLevel),
    List(ListLevels),
    Remove(RemoveLevel),
}

impl LevelCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, LevelError> {
        let client = context.client();
        let level_client = LevelClient::new(&client);

        let response = match self {
            LevelCommands::Set(set) => level_client.set(set).await?,
            LevelCommands::Show(get) => level_client.get(&get.name).await?,
            LevelCommands::List(list) => level_client.list(list).await?,
            LevelCommands::Remove(removal) => level_client.remove(&removal.name).await?,
        };

        Ok(LevelView::new(response).render().map(Into::into))
    }
}
