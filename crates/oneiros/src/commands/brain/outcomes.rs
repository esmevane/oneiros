use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum BrainOutcomes {
    #[outcome(transparent)]
    Replay(#[from] ReplayBrainOutcomes),
}
