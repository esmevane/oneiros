mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ListStorageOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ListStorage;

impl ListStorage {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ListStorageOutcomes>, StorageCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let entries = client.list_storage(&context.ticket_token()?).await?;

        if entries.is_empty() {
            outcomes.emit(ListStorageOutcomes::NoEntries);
        } else {
            outcomes.emit(ListStorageOutcomes::Entries(entries));
        }

        Ok(outcomes)
    }
}
