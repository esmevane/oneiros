use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::*;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum AddMemoryOutcomes {
    #[outcome(message("Memory added: {}", .0.ref_token()), prompt("Memory recorded: {}", .0.ref_token()))]
    MemoryAdded(Memory),
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
    ) -> Result<
        (
            Outcomes<AddMemoryOutcomes>,
            Vec<PressureSummary>,
            Option<RefToken>,
        ),
        MemoryCommandError,
    > {
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
        let ref_token = add_response.ref_token();
        let memory: Memory = add_response.data()?;

        outcomes.emit(AddMemoryOutcomes::MemoryAdded(memory));

        Ok((outcomes, summaries, ref_token))
    }
}
