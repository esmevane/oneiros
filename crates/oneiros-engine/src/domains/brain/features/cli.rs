use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum BrainCommands {
    Create(CreateBrain),
    Get(GetBrain),
    List(ListBrains),
}

impl BrainCommands {
    pub async fn execute(
        &self,
        context: &SystemContext,
    ) -> Result<Rendered<Responses>, BrainError> {
        let client = context.client();
        let brain_client = BrainClient::new(&client);

        let response = match self {
            BrainCommands::Create(create) => brain_client.create(create).await?,
            BrainCommands::Get(get) => brain_client.get(&get.name).await?,
            BrainCommands::List(list) => brain_client.list(list).await?,
        };

        let prompt = match &response {
            BrainResponse::Created(wrapped) => format!("Brain '{}' created.", wrapped.data.name),
            BrainResponse::Found(wrapped) => format!("Brain '{}'", wrapped.data.name),
            BrainResponse::Listed(listed) => {
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
