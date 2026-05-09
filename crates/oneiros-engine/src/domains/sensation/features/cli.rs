use clap::Subcommand;

use crate::*;

/// CLI subcommands for the sensation domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct. The
/// dispatcher passes the wrapper through to the client without rebuilding,
/// since the operation type *is* the domain command.
#[derive(Debug, Subcommand)]
pub(crate) enum SensationCommands {
    Set(SetSensation),
    Show(GetSensation),
    List(ListSensations),
    Remove(RemoveSensation),
}

impl SensationCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, SensationError> {
        let client = Client::from_config(config)?;
        let sensation_client = SensationClient::new(&client);

        let response = match self {
            Self::Set(setting) => sensation_client.set(setting).await?,
            Self::Show(lookup) => sensation_client.get(lookup).await?,
            Self::List(listing) => sensation_client.list(listing).await?,
            Self::Remove(removal) => sensation_client.remove(removal).await?,
        };

        Ok(SensationView::new(response).render().map(Into::into))
    }
}
