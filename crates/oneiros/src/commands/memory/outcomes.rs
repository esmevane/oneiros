use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum MemoryOutcomes {
    #[outcome(transparent)]
    Add(#[from] AddMemoryOutcomes),
    #[outcome(transparent)]
    List(#[from] ListMemoriesOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowMemoryOutcomes),
}
