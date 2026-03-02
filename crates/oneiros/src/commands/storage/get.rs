use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};
use std::path::PathBuf;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum GetStorageOutcomes {
    #[outcome(message("Downloaded '{0}' to {1:?}."))]
    ContentWritten(StorageKey, PathBuf),
}

#[derive(Clone, Args)]
pub struct GetStorage {
    /// The storage key.
    key: StorageKey,

    /// Output file path.
    output: PathBuf,
}

impl GetStorage {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<GetStorageOutcomes>, StorageCommandError> {
        let mut outcomes = Outcomes::new();

        let client = context.client();

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
