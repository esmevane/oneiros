mod outcomes;

use clap::Args;
use oneiros_client::{AddMemoryRequest, Client};
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::AddMemoryOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct AddMemory {
    /// The agent who holds this memory.
    agent: AgentName,

    /// The retention level for this memory.
    level: LevelName,

    /// The content of the memory.
    content: Content,
}

impl AddMemory {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<AddMemoryOutcomes>, MemoryCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let memory = client
            .add_memory(
                &context.ticket_token()?,
                AddMemoryRequest {
                    agent: self.agent.clone(),
                    level: self.level.clone(),
                    content: self.content.clone(),
                },
            )
            .await?;
        outcomes.emit(AddMemoryOutcomes::MemoryAdded(memory.id));

        Ok(outcomes)
    }
}
