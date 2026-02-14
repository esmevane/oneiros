mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::RemoveStorageOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct RemoveStorage {
    /// The storage key to remove.
    key: StorageKey,
}

impl RemoveStorage {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<RemoveStorageOutcomes>, StorageCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        client
            .remove_storage(&context.ticket_token()?, &self.key)
            .await?;

        outcomes.emit(RemoveStorageOutcomes::StorageRemoved(self.key.clone()));

        Ok(outcomes)
    }
}
