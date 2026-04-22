use clap::Subcommand;

use crate::*;

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
            ActorCommands::Create(creation) => actor_client.create(creation).await?,
            ActorCommands::Get(get) => actor_client.get(get).await?,
            ActorCommands::List(list) => actor_client.list(list).await?,
        };

        Ok(ActorView::new(response).render().map(Into::into))
    }
}
