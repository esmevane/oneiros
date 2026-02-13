use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum CognitionOutcomes {
    #[outcome(transparent)]
    Add(#[from] AddCognitionOutcomes),
    #[outcome(transparent)]
    List(#[from] ListCognitionsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowCognitionOutcomes),
}
