use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum LevelCommands {
    Set(SetLevel),
    Show(GetLevel),
    List(ListLevels),
    Remove(RemoveLevel),
}

impl LevelCommands {
    pub(crate) async fn execute(
        &self,
        client: &Client,
    ) -> Result<Rendered<Responses>, LevelError> {
        
        let level_client = LevelClient::new(client);

        let response = match self {
            LevelCommands::Set(set) => level_client.set(set).await?,
            LevelCommands::Show(get) => level_client.get(&get.name).await?,
            LevelCommands::List(list) => level_client.list(list).await?,
            LevelCommands::Remove(removal) => level_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            LevelResponse::LevelSet(name) => LevelView::confirmed("set", name).to_string(),
            LevelResponse::LevelDetails(wrapped) => LevelView::detail(&wrapped.data).to_string(),
            LevelResponse::Levels(listed) => {
                let table = LevelView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            LevelResponse::NoLevels => format!("{}", "No levels configured.".muted()),
            LevelResponse::LevelRemoved(name) => LevelView::confirmed("removed", name).to_string(),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
