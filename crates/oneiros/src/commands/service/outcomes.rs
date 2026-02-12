use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ServiceOutcomes {
    #[outcome(transparent)]
    Run(#[from] RunServiceOutcomes),
    #[outcome(transparent)]
    Status(#[from] ServiceStatusOutcomes),
}
