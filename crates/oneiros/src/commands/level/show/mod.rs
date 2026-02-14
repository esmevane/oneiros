mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowLevelOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowLevel {
    /// The level name to display.
    name: LevelName,
}

impl ShowLevel {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowLevelOutcomes>, LevelCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let info = client
            .get_level(&context.ticket_token()?, &self.name)
            .await?;
        outcomes.emit(ShowLevelOutcomes::LevelDetails(info));

        Ok(outcomes)
    }
}
