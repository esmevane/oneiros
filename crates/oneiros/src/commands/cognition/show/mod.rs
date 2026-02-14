mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowCognitionOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowCognition {
    /// The cognition ID to display.
    id: CognitionId,
}

impl ShowCognition {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowCognitionOutcomes>, CognitionCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let cognition = client
            .get_cognition(&context.ticket_token()?, &self.id)
            .await?;
        outcomes.emit(ShowCognitionOutcomes::CognitionDetails(cognition));

        Ok(outcomes)
    }
}
