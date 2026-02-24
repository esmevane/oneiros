use clap::Args;
use oneiros_client::Client;
use oneiros_model::SensationName;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetSensationOutcomes {
    #[outcome(message("Sensation '{0}' set."))]
    SensationSet(SensationName),
}

#[derive(Clone, Args)]
pub struct SetSensation {
    /// The sensation name (identity).
    pub name: SensationName,

    /// A human-readable description of the sensation's purpose.
    #[arg(long, default_value = "")]
    pub description: Description,

    /// Guidance text for agents when creating experiences of this sensation.
    #[arg(long, default_value = "")]
    pub prompt: Prompt,
}

impl SetSensation {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SetSensationOutcomes>, SensationCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .set_sensation(
                &context.ticket_token()?,
                SensationRecord::init(
                    self.description.clone(),
                    self.prompt.clone(),
                    Sensation {
                        name: self.name.clone(),
                    },
                ),
            )
            .await?;
        outcomes.emit(SetSensationOutcomes::SensationSet(info.name.clone()));

        Ok(outcomes)
    }
}
