use oneiros_model::StorageEntry;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ShowStorageOutcomes {
    #[outcome(message("Key: {}\n  Description: {}\n  Hash: {}", .0.key, .0.description, .0.hash))]
    StorageDetails(StorageEntry),
}
