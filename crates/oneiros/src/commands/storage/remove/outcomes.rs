use oneiros_model::StorageKey;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveStorageOutcomes {
    #[outcome(message("Storage entry '{0}' removed."))]
    StorageRemoved(StorageKey),
}
