mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::RemoveSensationOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RemoveSensation {
    /// The sensation name to remove.
    name: SensationName,
}

impl RemoveSensation {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemoveSensationOutcomes>, SensationCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        client
            .remove_sensation(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveSensationOutcomes::SensationRemoved(self.name.clone()));

        Ok(outcomes)
    }
}
