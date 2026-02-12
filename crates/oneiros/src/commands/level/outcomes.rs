use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
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
