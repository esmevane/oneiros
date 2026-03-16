use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::*;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetUrgeOutcomes {
    #[outcome(message("Urge '{0}' set."))]
    UrgeSet(UrgeName),
}

#[derive(Clone, Args)]
pub struct SetUrge {
    /// The urge name (identity).
    pub name: UrgeName,

    /// A human-readable description of the urge's purpose.
    #[arg(long, default_value = "")]
    pub description: Description,

    /// Guidance text for agents when logging cognition with this urge.
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

impl SetUrge {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<SetUrgeOutcomes>, Vec<PressureSummary>), UrgeCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client
            .set_urge(
                &context.ticket_token()?,
                Urge::init(
                    self.name.clone(),
                    self.description.clone(),
                    self.prompt.clone(),
                ),
            )
            .await?;
        let summaries = response.pressure_summaries();
        let info: Urge = response.data()?;
        outcomes.emit(SetUrgeOutcomes::UrgeSet(info.name.clone()));

        Ok((outcomes, summaries))
    }
}
