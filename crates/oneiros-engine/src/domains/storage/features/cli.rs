use clap::Subcommand;
use std::path::PathBuf;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum StorageCommands {
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
    pub(crate) async fn execute(
        &self,
        client: &Client,
    ) -> Result<Rendered<Responses>, StorageError> {
        
        let storage_client = StorageClient::new(client);

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
            StorageResponse::StorageSet(wrapped) => {
                StorageView::confirmed("stored", &wrapped.data.key).to_string()
            }
            StorageResponse::StorageDetails(wrapped) => {
                StorageView::detail(&wrapped.data).to_string()
            }
            StorageResponse::Entries(listed) => {
                let table = StorageView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
            StorageResponse::NoEntries => format!("{}", "No storage entries.".muted()),
            StorageResponse::StorageRemoved(key) => {
                StorageView::confirmed("removed", key).to_string()
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
