use clap::Args;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct MemoryDetail(pub Memory);

impl core::fmt::Display for MemoryDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_detail())
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowMemoryOutcomes {
    #[outcome(
        message("{0}"),
        prompt(
            "Is this grounded in specific cognitions? Mark it with `oneiros experience create <agent> grounds <description>`."
        )
    )]
    MemoryDetails(MemoryDetail),
}

#[derive(Clone, Args)]
pub struct ShowMemory {
    /// The memory ID (full UUID, 8+ character prefix, or ref:token).
    id: PrefixId,
}

impl ShowMemory {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowMemoryOutcomes>, MemoryCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
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
        outcomes.emit(ShowMemoryOutcomes::MemoryDetails(MemoryDetail(memory)));

        Ok(outcomes)
    }
}
