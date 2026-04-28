use clap::Subcommand;

use crate::*;

/// CLI subcommands for the actor domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct.
#[derive(Debug, Subcommand)]
pub enum ActorCommands {
    Create(CreateActor),
    Get(GetActor),
    List(ListActors),
}

impl ActorCommands {
    pub async fn execute(
        &self,
        context: &SystemContext,
    ) -> Result<Rendered<Responses>, ActorError> {
        let client = context.client();
        let actor_client = ActorClient::new(&client);

        let response = match self {
            Self::Create(creation) => actor_client.create(creation).await?,
            Self::Get(lookup) => actor_client.get(lookup).await?,
            Self::List(listing) => actor_client.list(listing).await?,
        };

        Ok(ActorView::new(response).render().map(Into::into))
    }
}
