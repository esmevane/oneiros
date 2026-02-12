mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListLevelsOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListLevels;

impl ListLevels {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<ListLevelsOutcomes>, LevelCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let levels = client.list_levels(&context.ticket_token()?).await?;

        if levels.is_empty() {
            outcomes.emit(ListLevelsOutcomes::NoLevels);
        } else {
            outcomes.emit(ListLevelsOutcomes::Levels(levels));
        }

        Ok(outcomes)
    }
}
