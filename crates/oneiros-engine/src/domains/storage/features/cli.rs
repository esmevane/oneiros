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
    List(ListStorage),
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
            StorageCommands::List(listing) => storage_client.list(listing).await?,
            StorageCommands::Remove(remove) => storage_client.remove(remove).await?,
        };

        let prompt = match &response {
            StorageResponse::StorageSet(wrapped) => format!("Stored '{}'.", wrapped.data.key),
            StorageResponse::StorageDetails(wrapped) => {
                format!(
                    "Key: {}\n  Description: {}\n  Hash: {}",
                    wrapped.data.key, wrapped.data.description, wrapped.data.hash
                )
            }
            StorageResponse::Entries(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for wrapped in &listed.items {
                    out.push_str(&format!(
                        "  {} — {}\n    hash: {}\n\n",
                        wrapped.data.key, wrapped.data.description, wrapped.data.hash
                    ));
                }
                out
            }
            StorageResponse::NoEntries => "No storage entries.".to_string(),
            StorageResponse::StorageRemoved(key) => format!("Storage entry '{key}' removed."),
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
