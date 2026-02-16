mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowSensationOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowSensation {
    /// The sensation name to display.
    name: SensationName,
}

impl ShowSensation {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowSensationOutcomes>, SensationCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .get_sensation(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(ShowSensationOutcomes::SensationDetails(info));

        Ok(outcomes)
    }
}
