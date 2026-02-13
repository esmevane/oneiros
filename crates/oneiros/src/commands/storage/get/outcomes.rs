use oneiros_model::StorageKey;
use oneiros_outcomes::Outcome;
use std::path::PathBuf;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum GetStorageOutcomes {
    #[outcome(message("Downloaded '{0}' to {1:?}."))]
    ContentWritten(StorageKey, PathBuf),
}
