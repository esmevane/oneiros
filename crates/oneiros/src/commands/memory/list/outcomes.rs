use oneiros_model::Memory;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ListMemoriesOutcomes {
    #[outcome(message("No memories found."))]
    NoMemories,

    #[outcome(message("Memories: {0:?}"))]
    Memories(Vec<Memory>),
}
