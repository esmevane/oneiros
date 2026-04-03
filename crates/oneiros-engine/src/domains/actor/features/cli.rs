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
            ActorCommands::Get(get) => actor_client.get(&get.id).await?,
            ActorCommands::List(list) => actor_client.list(list).await?,
        };

        let prompt = match &response {
            ActorResponse::Created(wrapped) => format!("Actor '{}' created.", wrapped.data.name),
            ActorResponse::Found(wrapped) => {
                format!("Actor '{}' ({})", wrapped.data.name, wrapped.data.id)
            }
            ActorResponse::Listed(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    out.push_str(&format!("  {}\n\n", wrapped.data.name));
                }
                out
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
