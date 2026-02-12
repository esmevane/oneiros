use crate::*;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum StorageOutcomes {
    #[outcome(transparent)]
    Set(#[from] SetStorageOutcomes),
    #[outcome(transparent)]
    Get(#[from] GetStorageOutcomes),
    #[outcome(transparent)]
    Remove(#[from] RemoveStorageOutcomes),
    #[outcome(transparent)]
    List(#[from] ListStorageOutcomes),
    #[outcome(transparent)]
    Show(#[from] ShowStorageOutcomes),
}
