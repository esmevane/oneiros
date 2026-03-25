use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum NatureCommands {
    Set(Nature),
    Show { name: NatureName },
    List,
    Remove { name: NatureName },
}

impl NatureCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, NatureError> {
        let client = context.client();
        let nature_client = NatureClient::new(&client);

        let response = match self {
            NatureCommands::Set(nature) => nature_client.set(nature).await?,
            NatureCommands::Show { name } => nature_client.get(name).await?,
            NatureCommands::List => nature_client.list().await?,
            NatureCommands::Remove { name } => nature_client.remove(name).await?,
        };

        let prompt = match &response {
            NatureResponse::NatureSet(name) => format!("Nature '{name}' set."),
            NatureResponse::NatureDetails(n) => format!("Nature details: {n:?}"),
            NatureResponse::Natures(natures) => format!("Natures: {natures:?}"),
            NatureResponse::NoNatures => "No natures configured.".to_string(),
            NatureResponse::NatureRemoved(name) => format!("Nature '{name}' removed."),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
