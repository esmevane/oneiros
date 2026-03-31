use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ActorCommands {
    Create(CreateActor),
    Get(GetActor),
    List,
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
            ActorCommands::Get(get) => actor_client.get(&get.id).await?,
            ActorCommands::List => actor_client.list().await?,
        };

        let prompt = match &response {
            ActorResponse::Created(actor) => format!("Actor '{}' created.", actor.name),
            ActorResponse::Found(actor) => format!("Actor '{}' ({})", actor.name, actor.id),
            ActorResponse::Listed(actors) => format!("{} actor(s) found.", actors.len()),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
