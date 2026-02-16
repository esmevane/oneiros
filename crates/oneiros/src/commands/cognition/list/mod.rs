mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListCognitionsOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListCognitions {
    /// Filter by agent name.
    #[arg(long)]
    agent: Option<AgentName>,

    /// Filter by texture name.
    #[arg(long)]
    texture: Option<TextureName>,
}

impl ListCognitions {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListCognitionsOutcomes>, CognitionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let cognitions = client
            .list_cognitions(
                &context.ticket_token()?,
                self.agent.as_ref(),
                self.texture.as_ref(),
            )
            .await?;

        if cognitions.is_empty() {
            outcomes.emit(ListCognitionsOutcomes::NoCognitions);
        } else {
            outcomes.emit(ListCognitionsOutcomes::Cognitions(outcomes::CognitionList(
                cognitions,
            )));
        }

        Ok(outcomes)
    }
}
