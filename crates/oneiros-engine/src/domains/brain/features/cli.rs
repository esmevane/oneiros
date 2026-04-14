use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum BrainCommands {
    Create(CreateBrain),
    Get(GetBrain),
    List(ListBrains),
}

impl BrainCommands {
    pub(crate) async fn execute(
        &self,
        client: &Client,
    ) -> Result<Rendered<Responses>, BrainError> {
        
        let brain_client = BrainClient::new(client);

        let response = match self {
            BrainCommands::Create(create) => brain_client.create(create).await?,
            BrainCommands::Get(get) => brain_client.get(&get.name).await?,
            BrainCommands::List(list) => brain_client.list(list).await?,
        };

        let prompt = match &response {
            BrainResponse::Created(wrapped) => {
                BrainView::confirmed("created", &wrapped.data.name).to_string()
            }
            BrainResponse::Found(wrapped) => BrainView::detail(&wrapped.data).to_string(),
            BrainResponse::Listed(listed) => {
                let table = BrainView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
