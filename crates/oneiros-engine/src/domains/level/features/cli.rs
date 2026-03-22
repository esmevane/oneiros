use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum LevelCommands {
    Set(Level),
    Show { name: LevelName },
    List,
    Remove { name: LevelName },
}

impl LevelCommands {
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, LevelError> {
        let client = context.client();
        let level_client = LevelClient::new(&client);

        let result = match self {
            LevelCommands::Set(level) => level_client.set(level).await?.into(),
            LevelCommands::Show { name } => level_client.get(name).await?.into(),
            LevelCommands::List => level_client.list().await?.into(),
            LevelCommands::Remove { name } => level_client.remove(name).await?.into(),
        };

        Ok(result)
    }
}
