mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::RemoveNatureOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RemoveNature {
    /// The nature name to remove.
    name: NatureName,
}

impl RemoveNature {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemoveNatureOutcomes>, NatureCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        client
            .remove_nature(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveNatureOutcomes::NatureRemoved(self.name.clone()));

        Ok(outcomes)
    }
}
