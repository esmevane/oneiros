mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::SetNatureOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct SetNature {
    /// The nature name (identity).
    pub(crate) name: NatureName,

    /// A human-readable description of the nature's purpose.
    #[arg(long, default_value = "")]
    pub(crate) description: Description,

    /// Guidance text for agents when creating connections of this nature.
    #[arg(long, default_value = "")]
    pub(crate) prompt: Prompt,
}

impl SetNature {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SetNatureOutcomes>, NatureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .set_nature(
                &context.ticket_token()?,
                Nature {
                    name: self.name.clone(),
                    description: self.description.clone(),
                    prompt: self.prompt.clone(),
                },
            )
            .await?;
        outcomes.emit(SetNatureOutcomes::NatureSet(info.name));

        Ok(outcomes)
    }
}
