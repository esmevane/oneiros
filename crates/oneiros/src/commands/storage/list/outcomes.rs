use oneiros_model::StorageEntry;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListStorageOutcomes {
    #[outcome(message("No storage entries."))]
    NoEntries,

    #[outcome(message("Storage entries: {0:?}"))]
    Entries(Vec<StorageEntry>),
}
