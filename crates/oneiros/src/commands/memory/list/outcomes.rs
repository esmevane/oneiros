use oneiros_model::Memory;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListMemoriesOutcomes {
    #[outcome(message("No memories found."))]
    NoMemories,

    #[outcome(message("Memories: {0:?}"))]
    Memories(Vec<Memory>),
}
