use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::*;

use crate::*;

#[derive(Clone, serde::Serialize)]
pub struct MemoryAddedResult {
    pub id: MemoryId,
    #[serde(skip)]
    pub ref_token: RefToken,
    #[serde(skip)]
    pub gauge: String,
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AddMemoryOutcomes {
    #[outcome(message("Memory added: {}", .0.ref_token), prompt("{}", .0.gauge))]
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
    ) -> Result<(Outcomes<AddMemoryOutcomes>, Vec<PressureSummary>), MemoryCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let token = context.ticket_token()?;

        let add_response = client
            .add_memory(
                &token,
                AddMemoryRequest {
                    agent: self.agent.clone(),
                    level: self.level.clone(),
                    content: self.content.clone(),
                },
            )
            .await?;
        let summaries = add_response.pressure_summaries();
        let memory: Memory = add_response.data()?;

        let all: Vec<Memory> = client
            .list_memories(&token, Some(&self.agent), None)
            .await?
            .data()?;
        let gauge = crate::gauge::memory_gauge(&self.agent, &all);

        let ref_token = memory.ref_token();

        outcomes.emit(AddMemoryOutcomes::MemoryAdded(MemoryAddedResult {
            id: memory.id,
            ref_token,
            gauge,
        }));

        Ok((outcomes, summaries))
    }
}
