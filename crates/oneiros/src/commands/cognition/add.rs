use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct CognitionAddedResult {
    pub id: CognitionId,
    #[serde(skip)]
    pub ref_token: RefToken,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AddCognitionOutcomes {
    #[outcome(message("Cognition added: {}", .0.ref_token), prompt("{}", .0.gauge))]
    CognitionAdded(CognitionAddedResult),
}

#[derive(Clone, Args)]
pub struct AddCognition {
    /// The agent who is thinking this thought.
    agent: AgentName,

    /// The texture (cognitive category) of this thought.
    texture: TextureName,

    /// The content of the thought.
    content: Content,
}

impl AddCognition {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<AddCognitionOutcomes>, CognitionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let cognition = client
            .add_cognition(
                &token,
                AddCognitionRequest {
                    agent: self.agent.clone(),
                    texture: self.texture.clone(),
                    content: self.content.clone(),
                },
            )
            .await?;

        let all = client
            .list_cognitions(&token, Some(&self.agent), None)
            .await?;
        let gauge = crate::gauge::cognition_gauge(&self.agent, &all);

        let ref_token = cognition.ref_token();

        outcomes.emit(AddCognitionOutcomes::CognitionAdded(CognitionAddedResult {
            id: cognition.id,
            ref_token,
            gauge,
        }));

        Ok(outcomes)
    }
}
