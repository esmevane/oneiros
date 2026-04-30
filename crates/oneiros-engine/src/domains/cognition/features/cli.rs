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
        context: &ProjectLog,
    ) -> Result<Rendered<Responses>, CognitionError> {
        let client = context.client();
        let cognition_client = CognitionClient::new(&client);

        let (response, request) = match self {
            Self::Add(addition) => (
                cognition_client.add(addition).await?,
                CognitionRequest::AddCognition(addition.clone()),
            ),
            Self::Show(lookup) => (
                cognition_client.get(lookup).await?,
                CognitionRequest::GetCognition(lookup.clone()),
            ),
            Self::List(listing) => (
                cognition_client.list(listing).await?,
                CognitionRequest::ListCognitions(listing.clone()),
            ),
        };

        Ok(CognitionView::new(response, &request)
            .render()
            .map(Into::into))
    }
}
