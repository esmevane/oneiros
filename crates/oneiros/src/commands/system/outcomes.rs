use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum SystemOutcomes {
    #[outcome(transparent)]
    InitOutcome(#[from] InitSystemOutcomes),
}
