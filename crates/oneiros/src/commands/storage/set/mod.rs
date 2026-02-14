mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;
use std::path::PathBuf;

pub(crate) use outcomes::SetStorageOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct SetStorage {
    /// The storage key.
    key: StorageKey,

    /// Path to the file to store.
    file: PathBuf,

    /// A human-readable description.
    #[arg(long, default_value = "")]
    description: String,
}

impl SetStorage {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<SetStorageOutcomes>, StorageCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());
        let data = context.files().read(&self.file)?;

        let entry = client
            .set_storage(&context.ticket_token()?, &self.key, data, &self.description)
            .await?;

        outcomes.emit(SetStorageOutcomes::StorageSet(entry.key));

        Ok(outcomes)
    }
}
