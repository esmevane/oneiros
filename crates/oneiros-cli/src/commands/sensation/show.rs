use clap::Args;
use oneiros_model::Sensation;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowSensationOutcomes {
    #[outcome(message("Sensation details: {0:?}"))]
    SensationDetails(Sensation),
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
    ) -> Result<(Outcomes<ShowSensationOutcomes>, Vec<PressureSummary>), SensationCommandError>
    {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client
            .get_sensation(&context.ticket_token()?, &self.name)
            .await?;
        let summaries = response.pressure_summaries();
        let info: Sensation = response.data()?;
        outcomes.emit(ShowSensationOutcomes::SensationDetails(info));

        Ok((outcomes, summaries))
    }
}
