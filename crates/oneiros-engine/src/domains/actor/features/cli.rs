use clap::Subcommand;

use crate::*;

/// CLI subcommands for the actor domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct.
#[derive(Debug, Subcommand)]
pub(crate) enum ActorCommands {
    Create(CreateActor),
    Get(GetActor),
    List(ListActors),
}

impl ActorCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, ActorError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Create(creation) => creation.execute_request(&client).await?,
            Self::Get(lookup) => lookup.execute_request(&client).await?,
            Self::List(listing) => listing.execute_request(&client).await?,
        };

        let response: ActorResponse = serde_json::from_slice(&bytes)?;
        Ok(ActorView::new(response).render().map(Into::into))
    }
}
