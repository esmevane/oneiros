use clap::Args;
use oneiros_client::Client;
use oneiros_model::*;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct MemoryList(pub Vec<Record<MemoryId, Memory>>);

impl core::fmt::Display for MemoryList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self
            .0
            .iter()
            .map(|memory| format!("{memory}"))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{display}")
    }
}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListMemoriesOutcomes {
    #[outcome(message("No memories found."))]
    NoMemories,

    #[outcome(
        message("{0}"),
        prompt(
            "Which of these are still true? Has anything shifted since they were consolidated?"
        )
    )]
    Memories(MemoryList),
}

#[derive(Clone, Args)]
pub struct ListMemories {
    /// Filter by agent name.
    #[arg(long)]
    agent: Option<AgentName>,

    /// Filter by level name.
    #[arg(long)]
    level: Option<LevelName>,
}

impl ListMemories {
    pub async fn run(
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
            outcomes.emit(ListMemoriesOutcomes::Memories(MemoryList(memories)));
        }

        Ok(outcomes)
    }
}
