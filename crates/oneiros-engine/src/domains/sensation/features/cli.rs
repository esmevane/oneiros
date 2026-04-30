use clap::Subcommand;

use crate::*;

/// CLI subcommands for the sensation domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct. The
/// dispatcher passes the wrapper through to the client without rebuilding,
/// since the operation type *is* the domain command.
#[derive(Debug, Subcommand)]
pub enum SensationCommands {
    Set(SetSensation),
    Show(GetSensation),
    List(ListSensations),
    Remove(RemoveSensation),
}

impl SensationCommands {
    pub async fn execute(
        &self,
        context: &ProjectLog,
    ) -> Result<Rendered<Responses>, SensationError> {
        let client = context.client();
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
