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
            CognitionResponse::CognitionAdded(cognition) => {
                let ref_token = RefToken::new(Ref::cognition(cognition.id));
                format!("Cognition recorded: {ref_token}")
            }
            CognitionResponse::CognitionDetails(Cognition {
                id: _,
                agent_id: _,
                texture,
                content,
                created_at: _,
            }) => format!("[{texture}] {content}"),
            CognitionResponse::Cognitions(list) => format!("{} cognitions.", list.len()),
            CognitionResponse::NoCognitions => "No cognitions.".to_string(),
        };

        let envelope = match response.clone() {
            CognitionResponse::CognitionAdded(Cognition {
                id,
                agent_id: _,
                texture: _,
                content: _,
                created_at: _,
            }) => Response::new(response.into()).with_ref_token(RefToken::new(Ref::cognition(id))),
            otherwise => Response::new(otherwise.into()),
        };

        Ok(Rendered::new(envelope, prompt, String::new()))
    }
}
