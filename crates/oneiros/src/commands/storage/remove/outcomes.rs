use oneiros_model::StorageKey;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum RemoveStorageOutcomes {
    #[outcome(message("Storage entry '{0}' removed."))]
    StorageRemoved(StorageKey),
}
