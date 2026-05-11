use clap::Subcommand;
use std::io::Write;
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
    /// Download the raw bytes for a stored key. Writes to `--out` when
    /// provided, otherwise to stdout.
    Get {
        key: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    List(ListStorage),
    Remove(RemoveStorage),
}

impl StorageCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, StorageError> {
        let client = Client::from_config(config)?;
        let storage_client = StorageClient::new(&client);

        let response = match self {
            Self::Set {
                key,
                file,
                description,
            } => {
                let data = config.platform().read(file)?;
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
            Self::Get { key, out } => {
                let bytes = storage_client.get_content(&StorageKey::new(key)).await?;
                match out {
                    Some(path) => {
                        let len = bytes.len();
                        config.platform().write(path, &bytes)?;
                        return Ok(Rendered::new(
                            Responses::Storage(StorageResponse::NoEntries),
                            format!("Wrote {} bytes to {}", len, path.display()),
                            String::new(),
                        ));
                    }
                    None => {
                        std::io::stdout().write_all(&bytes)?;
                        return Ok(Rendered::silent(Responses::Storage(
                            StorageResponse::NoEntries,
                        )));
                    }
                }
            }
            Self::List(listing) => storage_client.list(listing).await?,
            Self::Remove(removal) => storage_client.remove(removal).await?,
        };

        Ok(StorageView::new(response).render().map(Into::into))
    }
}
