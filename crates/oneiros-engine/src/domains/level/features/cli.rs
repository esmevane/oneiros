use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum LevelCommands {
    Set(Level),
    Show(GetLevel),
    List,
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
            LevelCommands::Set(level) => level_client.set(level).await?,
            LevelCommands::Show(get) => level_client.get(&get.name).await?,
            LevelCommands::List => level_client.list().await?,
            LevelCommands::Remove(removal) => level_client.remove(&removal.name).await?,
        };

        let prompt = match &response {
            LevelResponse::LevelSet(name) => format!("Level '{name}' set."),
            LevelResponse::LevelDetails(l) => {
                format!(
                    "Level '{}'\n  Description: {}\n  Prompt: {}",
                    l.name, l.description, l.prompt
                )
            }
            LevelResponse::Levels(levels) => format!("Levels: {levels:?}"),
            LevelResponse::NoLevels => "No levels configured.".to_string(),
            LevelResponse::LevelRemoved(name) => format!("Level '{name}' removed."),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
