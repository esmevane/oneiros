use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowMemoryOutcomes {
    #[outcome(
        message("Memory {}\n  Agent: {}\n  Level: {}\n  Content: {}\n  Created: {}", .0.id, .0.agent_id, .0.level, .0.content, .0.created_at),
        prompt("Is this grounded in specific cognitions? Mark it with `oneiros experience create <agent> grounds <description>`.")
    )]
    MemoryDetails(Memory),
}

#[derive(Clone, Args)]
pub struct ShowMemory {
    /// The memory ID (full UUID or 8+ character prefix).
    id: PrefixId,
}

impl ShowMemory {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowMemoryOutcomes>, MemoryCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let token = context.ticket_token()?;

        let id = match self.id.as_full_id() {
            Some(id) => MemoryId(id),
            None => {
                let all = client.list_memories(&token, None, None).await?;
                let ids: Vec<_> = all.iter().map(|m| m.id.0).collect();
                MemoryId(self.id.resolve(&ids)?)
            }
        };

        let memory = client.get_memory(&token, &id).await?;
        outcomes.emit(ShowMemoryOutcomes::MemoryDetails(memory));

        Ok(outcomes)
    }
}
