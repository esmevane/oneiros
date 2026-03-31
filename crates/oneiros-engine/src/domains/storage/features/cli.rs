use clap::Subcommand;
use std::path::PathBuf;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum StorageCommands {
    Set {
        key: String,
        file: PathBuf,
        #[arg(long, default_value = "")]
        description: String,
    },
    Show(GetStorage),
    List,
    Remove(RemoveStorage),
}

impl StorageCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, StorageError> {
        let client = context.client();
        let storage_client = StorageClient::new(&client);

        let response = match self {
            StorageCommands::Set {
                key,
                file,
                description,
            } => {
                let data = std::fs::read(file)?;
                storage_client
                    .upload(
                        &UploadStorage::builder()
                            .key(StorageKey::new(key))
                            .description(Description::new(description))
                            .data(data)
                            .build(),
                    )
                    .await?
            }
            StorageCommands::Show(get) => storage_client.show(get).await?,
            StorageCommands::List => storage_client.list().await?,
            StorageCommands::Remove(remove) => storage_client.remove(remove).await?,
        };

        let prompt = match &response {
            StorageResponse::StorageSet(e) => format!("Stored '{}'.", e.key),
            StorageResponse::StorageDetails(e) => {
                format!(
                    "Key: {}\n  Description: {}\n  Hash: {}",
                    e.key, e.description, e.hash
                )
            }
            StorageResponse::Entries(entries) => format!("Storage entries: {entries:?}"),
            StorageResponse::NoEntries => "No storage entries.".to_string(),
            StorageResponse::StorageRemoved(key) => format!("Storage entry '{key}' removed."),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
