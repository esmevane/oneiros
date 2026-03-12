use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum RemoveStorageOutcomes {
    #[outcome(message("Storage entry '{0}' removed."))]
    StorageRemoved(StorageKey),
}

#[derive(Clone, Args)]
pub struct RemoveStorage {
    /// The storage key to remove.
    key: StorageKey,
}

impl RemoveStorage {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemoveStorageOutcomes>, StorageCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        client
            .remove_storage(&context.ticket_token()?, &self.key)
            .await?;

        outcomes.emit(RemoveStorageOutcomes::StorageRemoved(self.key.clone()));

        Ok(outcomes)
    }
}
