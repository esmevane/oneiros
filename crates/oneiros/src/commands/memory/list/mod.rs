mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListMemoriesOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListMemories {
    /// Filter by agent name.
    #[arg(long)]
    agent: Option<AgentName>,

    /// Filter by level name.
    #[arg(long)]
    level: Option<LevelName>,
}

impl ListMemories {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListMemoriesOutcomes>, MemoryCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let memories = client
            .list_memories(
                &context.ticket_token()?,
                self.agent.as_ref(),
                self.level.as_ref(),
            )
            .await?;

        if memories.is_empty() {
            outcomes.emit(ListMemoriesOutcomes::NoMemories);
        } else {
            outcomes.emit(ListMemoriesOutcomes::Memories(outcomes::MemoryList(
                memories,
            )));
        }

        Ok(outcomes)
    }
}
