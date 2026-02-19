use oneiros_model::{Identity, Memory, MemoryId};
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize)]
#[serde(transparent)]
pub struct MemoryList(pub Vec<Identity<MemoryId, Memory>>);

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
