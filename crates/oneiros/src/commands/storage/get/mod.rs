mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;
use std::path::PathBuf;

pub(crate) use outcomes::GetStorageOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct GetStorage {
    /// The storage key.
    key: StorageKey,

    /// Output file path.
    output: PathBuf,
}

impl GetStorage {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<GetStorageOutcomes>, StorageCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        let content = client
            .get_storage_content(&context.ticket_token()?, &self.key)
            .await?;

        context.files().write(&self.output, &content)?;

        outcomes.emit(GetStorageOutcomes::ContentWritten(
            self.key.clone(),
            self.output.clone(),
        ));

        Ok(outcomes)
    }
}
