use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};
use std::path::PathBuf;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SetStorageOutcomes {
    #[outcome(message("Stored '{0}'."))]
    StorageSet(StorageKey),
}

#[derive(Clone, Args)]
pub struct SetStorage {
    /// The storage key.
    key: StorageKey,

    /// Path to the file to store.
    file: PathBuf,

    /// A human-readable description.
    #[arg(long, default_value = "")]
    description: String,
}

impl SetStorage {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SetStorageOutcomes>, StorageCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();
        let data = context.files().read(&self.file)?;

        let entry = client
            .set_storage(&context.ticket_token()?, &self.key, data, &self.description)
            .await?;

        outcomes.emit(SetStorageOutcomes::StorageSet(entry.key.clone()));

        Ok(outcomes)
    }
}
