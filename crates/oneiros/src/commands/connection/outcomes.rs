use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum ConnectionOutcomes {
    #[outcome(transparent)]
    Create(#[from] CreateConnectionOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveConnectionOutcomes),
    #[outcome(transparent)]
    List(#[from] ListConnectionsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowConnectionOutcomes),
}
