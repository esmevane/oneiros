use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(untagged)]
pub enum LevelOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetLevelOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveLevelOutcomes),
    #[outcome(transparent)]
    List(#[from] ListLevelsOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowLevelOutcomes),
}
