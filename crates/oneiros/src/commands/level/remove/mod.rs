mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::RemoveLevelOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RemoveLevel {
    /// The level name to remove.
    name: LevelName,
}

impl RemoveLevel {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<RemoveLevelOutcomes>, LevelCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        client
            .remove_level(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(RemoveLevelOutcomes::LevelRemoved(self.name.clone()));

        Ok(outcomes)
    }
}
