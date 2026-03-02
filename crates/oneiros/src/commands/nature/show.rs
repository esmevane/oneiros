use clap::Args;
use oneiros_model::Nature;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowNatureOutcomes {
    #[outcome(message("Nature details: {0:?}"))]
    NatureDetails(Nature),
}

#[derive(Clone, Args)]
pub struct ShowNature {
    /// The nature name to display.
    name: NatureName,
}

impl ShowNature {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowNatureOutcomes>, NatureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let info = client
            .get_nature(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(ShowNatureOutcomes::NatureDetails(info));

        Ok(outcomes)
    }
}
