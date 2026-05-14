use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum CognitionCommands {
    Add(AddCognition),
    Show(GetCognition),
    List(ListCognitions),
}

impl CognitionCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, CognitionError> {
        let client = Client::from_config(config)?;

        let (bytes, request) = match self {
            Self::Add(addition) => (
                addition.execute_request(&client).await?,
                CognitionRequest::AddCognition(addition.clone()),
            ),
            Self::Show(lookup) => (
                lookup.execute_request(&client).await?,
                CognitionRequest::GetCognition(lookup.clone()),
            ),
            Self::List(listing) => (
                listing.execute_request(&client).await?,
                CognitionRequest::ListCognitions(listing.clone()),
            ),
        };

        let response: CognitionResponse = serde_json::from_slice(&bytes)?;
        Ok(CognitionView::new(response, &request)
            .render()
            .map(Into::into))
    }
}
