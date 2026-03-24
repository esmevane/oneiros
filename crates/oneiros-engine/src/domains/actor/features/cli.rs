use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

#[derive(Debug, Subcommand)]
pub enum ActorCommands {
    Create {
        #[arg(long)]
        tenant_id: String,
        name: String,
    },
    Get {
        id: String,
    },
    List,
}

impl ActorCommands {
    pub async fn execute(
        &self,
        context: &SystemContext,
    ) -> Result<Rendered<Responses>, ActorError> {
        let response = match self {
            ActorCommands::Create { tenant_id, name } => {
                ActorService::create(
                    context,
                    tenant_id.parse::<TenantId>()?,
                    ActorName::new(name),
                )
                .await?
            }
            ActorCommands::Get { id } => ActorService::get(context, &id.parse::<ActorId>()?)?,
            ActorCommands::List => ActorService::list(context)?,
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
