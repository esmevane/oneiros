use oneiros_model::StorageEntry;
use oneiros_outcomes::Outcome;

#[derive(Clone, Outcome)]
pub enum ListStorageOutcomes {
    #[outcome(message("No storage entries."))]
    NoEntries,

    #[outcome(message("Storage entries: {0:?}"))]
    Entries(Vec<StorageEntry>),
}
