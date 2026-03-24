use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SensationCommands {
    Set(Sensation),
    Show { name: SensationName },
    List,
    Remove { name: SensationName },
}

impl SensationCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, SensationError> {
        let client = context.client();
        let sensation_client = SensationClient::new(&client);

        let response = match self {
            SensationCommands::Set(sensation) => sensation_client.set(sensation).await?,
            SensationCommands::Show { name } => sensation_client.get(name).await?,
            SensationCommands::List => sensation_client.list().await?,
            SensationCommands::Remove { name } => sensation_client.remove(name).await?,
        };

        let prompt = match &response {
            SensationResponse::SensationSet(name) => format!("Sensation '{name}' set."),
            SensationResponse::SensationDetails(s) => format!("Sensation details: {s:?}"),
            SensationResponse::Sensations(sensations) => format!("Sensations: {sensations:?}"),
            SensationResponse::NoSensations => "No sensations configured.".to_string(),
            SensationResponse::SensationRemoved(name) => format!("Sensation '{name}' removed."),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
