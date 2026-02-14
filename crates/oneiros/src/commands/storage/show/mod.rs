mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ShowStorageOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct ShowStorage {
    /// The storage key to display.
    key: StorageKey,
}

impl ShowStorage {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ShowStorageOutcomes>, StorageCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let entry = client
            .get_storage(&context.ticket_token()?, &self.key)
            .await?;

        outcomes.emit(ShowStorageOutcomes::StorageDetails(entry));

        Ok(outcomes)
    }
}
