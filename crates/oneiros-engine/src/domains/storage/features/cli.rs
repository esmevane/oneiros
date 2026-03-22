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
    Show {
        key: String,
    },
    List,
    Remove {
        key: String,
    },
}

impl StorageCommands {
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, StorageError> {
        let client = context.client();
        let storage_client = StorageClient::new(&client);

        let result = match self {
            StorageCommands::Set {
                key,
                file,
                description,
            } => {
                let data = std::fs::read(file)?;
                storage_client
                    .upload(&StorageKey::new(key), &Description::new(description), data)
                    .await?
                    .into()
            }
            StorageCommands::Show { key } => {
                storage_client.show(&StorageKey::new(key)).await?.into()
            }
            StorageCommands::List => storage_client.list().await?.into(),
            StorageCommands::Remove { key } => {
                storage_client.remove(&StorageKey::new(key)).await?.into()
            }
        };
        Ok(result)
    }
}
