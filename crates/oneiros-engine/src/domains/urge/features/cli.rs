use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum UrgeCommands {
    Set(Urge),
    Show { name: UrgeName },
    List,
    Remove { name: UrgeName },
}

impl UrgeCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, UrgeError> {
        let client = context.client();
        let urge_client = UrgeClient::new(&client);

        let response = match self {
            UrgeCommands::Set(urge) => urge_client.set(urge).await?,
            UrgeCommands::Show { name } => urge_client.get(name).await?,
            UrgeCommands::List => urge_client.list().await?,
            UrgeCommands::Remove { name } => urge_client.remove(name).await?,
        };

        let prompt = match &response {
            UrgeResponse::UrgeSet(name) => format!("Urge '{name}' set."),
            UrgeResponse::UrgeDetails(u) => {
                format!(
                    "Urge '{}'\n  Description: {}\n  Prompt: {}",
                    u.name, u.description, u.prompt
                )
            }
            UrgeResponse::Urges(urges) => format!("Urges: {urges:?}"),
            UrgeResponse::NoUrges => "No urges configured.".to_string(),
            UrgeResponse::UrgeRemoved(name) => format!("Urge '{name}' removed."),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
