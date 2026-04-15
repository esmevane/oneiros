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

        Ok(StorageView::new(response).render().map(Into::into))
    }
}
