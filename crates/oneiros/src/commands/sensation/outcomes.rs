use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum SensationOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetSensationOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveSensationOutcomes),
    #[outcome(transparent)]
    List(#[from] ListSensationsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowSensationOutcomes),
}
