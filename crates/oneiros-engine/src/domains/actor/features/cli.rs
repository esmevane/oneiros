use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum ActorCommands {
    Create(CreateActor),
    Get(GetActor),
    List(ListActors),
}

impl ActorCommands {
    pub(crate) async fn execute(
        &self,
        client: &Client,
    ) -> Result<Rendered<Responses>, ActorError> {
        
        let actor_client = ActorClient::new(client);

        let response = match self {
            ActorCommands::Create(creation) => actor_client.create(creation).await?,
            ActorCommands::Get(get) => actor_client.get(&get.id).await?,
            ActorCommands::List(list) => actor_client.list(list).await?,
        };

        let prompt = match &response {
            ActorResponse::Created(wrapped) => {
                ActorView::confirmed("created", &wrapped.data.name).to_string()
            }
            ActorResponse::Found(wrapped) => ActorView::detail(&wrapped.data).to_string(),
            ActorResponse::Listed(listed) => {
                let table = ActorView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
