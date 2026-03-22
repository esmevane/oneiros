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
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, SensationError> {
        let client = context.client();
        let sensation_client = SensationClient::new(&client);

        let result = match self {
            SensationCommands::Set(sensation) => sensation_client.set(sensation).await?.into(),
            SensationCommands::Show { name } => sensation_client.get(name).await?.into(),
            SensationCommands::List => sensation_client.list().await?.into(),
            SensationCommands::Remove { name } => sensation_client.remove(name).await?.into(),
        };

        Ok(result)
    }
}
