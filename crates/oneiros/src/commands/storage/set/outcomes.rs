use oneiros_model::StorageKey;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum SetStorageOutcomes {
    #[outcome(message("Stored '{0}'."))]
    StorageSet(StorageKey),
}
