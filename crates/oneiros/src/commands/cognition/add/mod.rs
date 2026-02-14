mod outcomes;

use clap::Args;
use oneiros_client::{AddCognitionRequest, Client};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::AddCognitionOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct AddCognition {
    /// The agent who is thinking this thought.
    agent: AgentName,

    /// The texture (cognitive category) of this thought.
    texture: TextureName,

    /// The content of the thought.
    content: Content,
}

impl AddCognition {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<AddCognitionOutcomes>, CognitionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let cognition = client
            .add_cognition(
                &context.ticket_token()?,
                AddCognitionRequest {
                    agent: self.agent.clone(),
                    texture: self.texture.clone(),
                    content: self.content.clone(),
                },
            )
            .await?;
        outcomes.emit(AddCognitionOutcomes::CognitionAdded(cognition.id));

        Ok(outcomes)
    }
}
