use clap::Subcommand;

use crate::contexts::SystemContext;
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
        let response = match self {
            ActorCommands::Create(creation) => {
                ActorService::create(context, creation.tenant_id, creation.name.clone()).await?
            }
            ActorCommands::Get(get) => ActorService::get(context, get.id).await?,
            ActorCommands::List => ActorService::list(context).await?,
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
