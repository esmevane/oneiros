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

        match self {
            CognitionCommands::Add(addition) => {
                let response = cognition_client
                    .add(
                        addition.agent.clone(),
                        addition.texture.clone(),
                        addition.content.clone(),
                    )
                    .await?;
                let ref_token = match &response {
                    CognitionResponse::CognitionAdded(c) => {
                        Some(RefToken::new(Ref::cognition(c.id)))
                    }
                    _ => None,
                };
                let prompt = ref_token
                    .as_ref()
                    .map(|rt| format!("Cognition recorded: {rt}"))
                    .unwrap_or_default();
                let mut envelope = Response::new(response.into());
                if let Some(rt) = ref_token {
                    envelope = envelope.with_ref_token(rt);
                }
                Ok(Rendered::new(envelope, prompt, String::new()))
            }
            CognitionCommands::Show(get) => {
                let response = cognition_client.get(&get.id).await?;
                let prompt = match &response {
                    CognitionResponse::CognitionDetails(c) => {
                        format!("[{}] {}", c.texture, c.content)
                    }
                    other => format!("{other:?}"),
                };
                Ok(Rendered::new(
                    Response::new(response.into()),
                    prompt,
                    String::new(),
                ))
            }
            CognitionCommands::List(listing) => {
                let response = cognition_client
                    .list(listing.agent.as_ref(), listing.texture.as_ref())
                    .await?;
                let prompt = match &response {
                    CognitionResponse::Cognitions(list) => format!("{} cognitions.", list.len()),
                    CognitionResponse::NoCognitions => "No cognitions.".to_string(),
                    other => format!("{other:?}"),
                };
                Ok(Rendered::new(
                    Response::new(response.into()),
                    prompt,
                    String::new(),
                ))
            }
        }
    }
}
