use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ProjectOutcomes {
    #[outcome(transparent)]
    Init(#[from] InitProjectOutcomes),
}
