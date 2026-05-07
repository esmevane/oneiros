use clap::Subcommand;
use std::path::PathBuf;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum StorageCommands {
    /// Upload a file to storage. Reads the bytes from `file` and
    /// constructs the protocol-level `UploadStorage` request.
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
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, StorageError> {
        let client = Client::new(config.base_url());
        let storage_client = StorageClient::new(&client);

        let response = match self {
            Self::Set {
                key,
                file,
                description,
            } => {
                let data = std::fs::read(file)?;
                storage_client
                    .upload(
                        &UploadStorage::builder_v1()
                            .key(StorageKey::new(key))
                            .description(Description::new(description))
                            .data(data)
                            .build()
                            .into(),
                    )
                    .await?
            }
            Self::Show(lookup) => storage_client.show(lookup).await?,
            Self::List(listing) => storage_client.list(listing).await?,
            Self::Remove(removal) => storage_client.remove(removal).await?,
        };

        Ok(StorageView::new(response).render().map(Into::into))
    }
}
