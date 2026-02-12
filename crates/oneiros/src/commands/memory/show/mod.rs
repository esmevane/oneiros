mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowMemoryOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowMemory {
    /// The memory ID to display.
    id: MemoryId,
}

impl ShowMemory {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<ShowMemoryOutcomes>, MemoryCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let memory = client
            .get_memory(&context.ticket_token()?, &self.id)
            .await?;
        outcomes.emit(ShowMemoryOutcomes::MemoryDetails(memory));

        Ok(outcomes)
    }
}
