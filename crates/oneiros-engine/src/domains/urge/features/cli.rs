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
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, UrgeError> {
        let client = context.client();
        let urge_client = UrgeClient::new(&client);

        let result = match self {
            UrgeCommands::Set(urge) => urge_client.set(urge).await?.into(),
            UrgeCommands::Show { name } => urge_client.get(name).await?.into(),
            UrgeCommands::List => urge_client.list().await?.into(),
            UrgeCommands::Remove { name } => urge_client.remove(name).await?.into(),
        };

        Ok(result)
    }
}
