use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum NatureOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetNatureOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveNatureOutcomes),
    #[outcome(transparent)]
    List(#[from] ListNaturesOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowNatureOutcomes),
}
