use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum SystemOutcomes {
    #[outcome(transparent)]
    InitOutcome(#[from] InitSystemOutcomes),
}
