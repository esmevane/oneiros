mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowNatureOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowNature {
    /// The nature name to display.
    name: NatureName,
}

impl ShowNature {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowNatureOutcomes>, NatureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .get_nature(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(ShowNatureOutcomes::NatureDetails(info));

        Ok(outcomes)
    }
}
