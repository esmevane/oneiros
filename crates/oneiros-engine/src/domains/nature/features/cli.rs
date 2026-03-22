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
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, NatureError> {
        let client = context.client();
        let nature_client = NatureClient::new(&client);

        let result = match self {
            NatureCommands::Set(nature) => nature_client.set(nature).await?.into(),
            NatureCommands::Show { name } => nature_client.get(name).await?.into(),
            NatureCommands::List => nature_client.list().await?.into(),
            NatureCommands::Remove { name } => nature_client.remove(name).await?.into(),
        };

        Ok(result)
    }
}
