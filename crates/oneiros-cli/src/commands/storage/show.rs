use clap::Args;
use oneiros_model::StorageEntry;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ShowStorageOutcomes {
    #[outcome(message("Key: {}\n  Description: {}\n  Hash: {}", .0.key, .0.description, .0.hash), prompt("Key: {}\n  Description: {}\n  Hash: {}", .0.key, .0.description, .0.hash))]
    StorageDetails(StorageEntry),
}

#[derive(Clone, Args)]
pub struct ShowStorage {
    /// The storage key to display.
    key: StorageKey,
}

impl ShowStorage {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<ShowStorageOutcomes>, Vec<PressureSummary>), StorageCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client
            .get_storage(&context.ticket_token()?, &self.key)
            .await?;
        let summaries = response.pressure_summaries();
        let entry: StorageEntry = response.data()?;

        outcomes.emit(ShowStorageOutcomes::StorageDetails(entry));

        Ok((outcomes, summaries))
    }
}
