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

        let prompt = match &response {
            CognitionResponse::CognitionAdded(wrapped) => wrapped
                .meta()
                .ref_token()
                .map(|ref_token| format!("Cognition recorded: {ref_token}"))
                .unwrap_or_default(),
            CognitionResponse::CognitionDetails(wrapped) => {
                format!("[{}] {}", wrapped.data.texture, wrapped.data.content)
            }
            CognitionResponse::Cognitions(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    let ref_token = wrapped
                        .meta()
                        .ref_token()
                        .map(|ref_token| ref_token.to_string())
                        .unwrap_or_default();

                    out.push_str(&format!(
                        "  [{}] {}\n    {}\n\n",
                        wrapped.data.texture, wrapped.data.content, ref_token
                    ));
                }
                out
            }
            CognitionResponse::NoCognitions => "No cognitions.".to_string(),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
