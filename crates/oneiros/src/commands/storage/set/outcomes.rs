use oneiros_model::StorageKey;
use oneiros_outcomes::Outcome;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetStorageOutcomes {
    #[outcome(message("Stored '{0}'."))]
    StorageSet(StorageKey),
}
