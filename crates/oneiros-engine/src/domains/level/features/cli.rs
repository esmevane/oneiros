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

        let prompt = match &response {
            LevelResponse::LevelSet(name) => format!("Level '{name}' set."),
            LevelResponse::LevelDetails(l) => {
                format!(
                    "Level '{}'\n  Description: {}\n  Prompt: {}",
                    l.name, l.description, l.prompt
                )
            }
            LevelResponse::Levels(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for level in &listed.items {
                    out.push_str(&format!("  {} — {}\n\n", level.name, level.description,));
                }
                out
            }
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
