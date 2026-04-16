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

        let (response, request) = match self {
            CognitionCommands::Add(addition) => {
                let response = cognition_client.add(addition).await?;
                (response, CognitionRequest::AddCognition(addition.clone()))
            }
            CognitionCommands::Show(get) => {
                let response = cognition_client.get(get).await?;
                (response, CognitionRequest::GetCognition(get.clone()))
            }
            CognitionCommands::List(listing) => {
                let response = cognition_client.list(listing).await?;
                (response, CognitionRequest::ListCognitions(listing.clone()))
            }
        };

        Ok(CognitionView::new(response, &request)
            .render()
            .map(Into::into))
    }
}
