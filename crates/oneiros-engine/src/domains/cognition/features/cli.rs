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
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, CognitionError> {
        let client = context.client();
        let cognition_client = CognitionClient::new(&client);

        let response = match self {
            CognitionCommands::Add(addition) => cognition_client.add(addition).await?,
            CognitionCommands::Show(get) => cognition_client.get(get).await?,
            CognitionCommands::List(listing) => cognition_client.list(listing).await?,
        };

        let prompt = match &response {
            CognitionResponse::CognitionAdded(wrapped) => CognitionView::recorded(wrapped),
            CognitionResponse::CognitionDetails(wrapped) => {
                CognitionView::detail(&wrapped.data).to_string()
            }
            CognitionResponse::Cognitions(listed) => {
                let table = CognitionView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            CognitionResponse::NoCognitions => format!("{}", "No cognitions.".muted()),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
