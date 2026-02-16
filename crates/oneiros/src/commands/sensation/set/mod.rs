mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::SetSensationOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct SetSensation {
    /// The sensation name (identity).
    pub(crate) name: SensationName,

    /// A human-readable description of the sensation's purpose.
    #[arg(long, default_value = "")]
    pub(crate) description: Description,

    /// Guidance text for agents when creating experiences of this sensation.
    #[arg(long, default_value = "")]
    pub(crate) prompt: Prompt,
}

impl SetSensation {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SetSensationOutcomes>, SensationCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .set_sensation(
                &context.ticket_token()?,
                Sensation {
                    name: self.name.clone(),
                    description: self.description.clone(),
                    prompt: self.prompt.clone(),
                },
            )
            .await?;
        outcomes.emit(SetSensationOutcomes::SensationSet(info.name));

        Ok(outcomes)
    }
}
