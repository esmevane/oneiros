use clap::Args;
use oneiros_model::StorageEntry;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ListStorageOutcomes {
    #[outcome(message("No storage entries."), prompt("No storage entries."))]
    NoEntries,

    #[outcome(message("Storage entries: {0:?}"), prompt("Storage entries: {0:?}"))]
    Entries(Vec<StorageEntry>),
}

#[derive(Clone, Args)]
pub struct ListStorage;

impl ListStorage {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<(Outcomes<ListStorageOutcomes>, Vec<PressureSummary>), StorageCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

        let response = client.list_storage(&context.ticket_token()?).await?;
        let summaries = response.pressure_summaries();
        let entries: Vec<StorageEntry> = response.data()?;

        if entries.is_empty() {
            outcomes.emit(ListStorageOutcomes::NoEntries);
        } else {
            outcomes.emit(ListStorageOutcomes::Entries(entries));
        }

        Ok((outcomes, summaries))
    }
}
