use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum CognitionCommands {
    Add(AddCognition),
    Show(GetCognition),
    List(ListCognitions),
}

impl CognitionCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, CognitionError> {
        let client = context.client();
        let cognition_client = CognitionClient::new(&client);

        let response = match self {
            CognitionCommands::Add(addition) => cognition_client.add(addition).await?,
            CognitionCommands::Show(get) => cognition_client.get(get).await?,
            CognitionCommands::List(listing) => cognition_client.list(listing).await?,
        };

        Ok(CognitionView::new(response, self).render().map(Into::into))
    }
}
