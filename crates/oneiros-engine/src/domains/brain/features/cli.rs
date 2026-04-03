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
            BrainResponse::Created(brain) => format!("Brain '{}' created.", brain.name),
            BrainResponse::Found(brain) => format!("Brain '{}'", brain.name),
            BrainResponse::Listed(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for brain in &listed.items {
                    out.push_str(&format!("  {}\n\n", brain.name));
                }
                out
            }
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
