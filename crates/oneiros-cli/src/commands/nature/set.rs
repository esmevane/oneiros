use clap::Args;
use oneiros_model::{Nature, NatureName};
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetNatureOutcomes {
    #[outcome(message("Nature '{0}' set."), prompt("Nature '{0}' set."))]
    NatureSet(NatureName),
}

#[derive(Clone, Args)]
pub struct SetNature {
    /// The nature name (identity).
    pub name: NatureName,

    /// A human-readable description of the nature's purpose.
    #[arg(long, default_value = "")]
    pub description: Description,

    /// Guidance text for agents when creating connections of this nature.
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

impl SetNature {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<SetNatureOutcomes>, Vec<PressureSummary>), NatureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client
            .set_nature(
                &context.ticket_token()?,
                Nature::init(
                    self.name.clone(),
                    self.description.clone(),
                    self.prompt.clone(),
                ),
            )
            .await?;
        let summaries = response.pressure_summaries();
        let info: Nature = response.data()?;
        outcomes.emit(SetNatureOutcomes::NatureSet(info.name.clone()));

        Ok((outcomes, summaries))
    }
}
