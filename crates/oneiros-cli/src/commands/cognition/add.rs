use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AddCognitionOutcomes {
    #[outcome(message("Cognition added: {}", .0.ref_token()))]
    CognitionAdded(Cognition),
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
    ) -> Result<
        (
            Outcomes<AddCognitionOutcomes>,
            Vec<PressureSummary>,
            Option<RefToken>,
        ),
        CognitionCommandError,
    > {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let token = context.ticket_token()?;

        let add_response = client
            .add_cognition(
                &token,
                AddCognitionRequest {
                    agent: self.agent.clone(),
                    texture: self.texture.clone(),
                    content: self.content.clone(),
                },
            )
            .await?;
        let summaries = add_response.pressure_summaries();
        let ref_token = add_response.ref_token();
        let cognition: Cognition = add_response.data()?;

        outcomes.emit(AddCognitionOutcomes::CognitionAdded(cognition));

        Ok((outcomes, summaries, ref_token))
    }
}
