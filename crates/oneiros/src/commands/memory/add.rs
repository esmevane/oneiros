use clap::Args;
use oneiros_client::Client;
use oneiros_model::MemoryId;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct MemoryAddedResult {
    pub id: MemoryId,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AddMemoryOutcomes {
    #[outcome(message("Memory added: {}", .0.id), prompt("{}", .0.gauge))]
    MemoryAdded(MemoryAddedResult),
}

#[derive(Clone, Args)]
pub struct AddMemory {
    /// The agent who holds this memory.
    agent: AgentName,

    /// The retention level for this memory.
    level: LevelName,

    /// The content of the memory.
    content: Content,
}

impl AddMemory {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<AddMemoryOutcomes>, MemoryCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let memory = client
            .add_memory(
                &token,
                AddMemoryRequest {
                    agent: self.agent.clone(),
                    level: self.level.clone(),
                    content: self.content.clone(),
                },
            )
            .await?;

        let all = client
            .list_memories(&token, Some(&self.agent), None)
            .await?;
        let gauge = crate::gauge::memory_gauge(&self.agent, &all);

        outcomes.emit(AddMemoryOutcomes::MemoryAdded(MemoryAddedResult {
            id: memory.id,
            gauge,
        }));

        Ok(outcomes)
    }
}
