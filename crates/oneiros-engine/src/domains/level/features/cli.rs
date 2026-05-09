use clap::Subcommand;

use crate::*;

/// CLI subcommands for the level domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct. The
/// dispatcher passes the wrapper through to the client without rebuilding,
/// since the operation type *is* the domain command.
#[derive(Debug, Subcommand)]
pub(crate) enum LevelCommands {
    Set(SetLevel),
    Show(GetLevel),
    List(ListLevels),
    Remove(RemoveLevel),
}

impl LevelCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, LevelError> {
        let client = Client::from_config(config)?;
        let level_client = LevelClient::new(&client);

        let response = match self {
            Self::Set(setting) => level_client.set(setting).await?,
            Self::Show(lookup) => level_client.get(lookup).await?,
            Self::List(listing) => level_client.list(listing).await?,
            Self::Remove(removal) => level_client.remove(removal).await?,
        };

        Ok(LevelView::new(response).render().map(Into::into))
    }
}
