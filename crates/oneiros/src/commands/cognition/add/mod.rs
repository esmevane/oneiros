mod outcomes;

use clap::Args;
use oneiros_client::{AddCognitionRequest, Client};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::{AddCognitionOutcomes, CognitionAddedResult};

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

        outcomes.emit(AddCognitionOutcomes::CognitionAdded(CognitionAddedResult {
            id: cognition.id,
            gauge,
        }));

        Ok(outcomes)
    }
}
