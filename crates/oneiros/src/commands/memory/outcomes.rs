use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum MemoryOutcomes {
    #[outcome(transparent)]
    Add(#[from] AddMemoryOutcomes),
    #[outcome(transparent)]
    List(#[from] ListMemoriesOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowMemoryOutcomes),
}
