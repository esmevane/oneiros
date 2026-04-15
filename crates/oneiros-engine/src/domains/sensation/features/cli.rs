use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SensationCommands {
    Set(SetSensation),
    Show(GetSensation),
    List(ListSensations),
    Remove(RemoveSensation),
}

impl SensationCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, SensationError> {
        let client = context.client();
        let sensation_client = SensationClient::new(&client);

        let response = match self {
            SensationCommands::Set(set) => sensation_client.set(set).await?,
            SensationCommands::Show(get) => sensation_client.get(&get.name).await?,
            SensationCommands::List(list) => sensation_client.list(list).await?,
            SensationCommands::Remove(removal) => sensation_client.remove(&removal.name).await?,
        };

        Ok(SensationView::new(response).render().map(Into::into))
    }
}
