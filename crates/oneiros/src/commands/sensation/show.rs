use clap::Args;
use oneiros_client::Client;
use oneiros_model::SensationRecord;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowSensationOutcomes {
    #[outcome(message("Sensation details: {0:?}"))]
    SensationDetails(SensationRecord),
}

#[derive(Clone, Args)]
pub struct ShowSensation {
    /// The sensation name to display.
    name: SensationName,
}

impl ShowSensation {
    pub async fn run(
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
