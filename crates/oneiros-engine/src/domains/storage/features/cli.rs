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
    pub fn execute(&self, context: &ProjectContext) -> Result<Responses, StorageError> {
        let result = match self {
            StorageCommands::Set {
                key,
                file,
                description,
            } => {
                let data = std::fs::read(file)?;
                StorageService::upload(
                    context,
                    StorageKey::new(key),
                    Description::new(description),
                    data,
                )?
                .into()
            }
            StorageCommands::Show { key } => {
                StorageService::show(context, &StorageKey::new(key))?.into()
            }
            StorageCommands::List => StorageService::list(context)?.into(),
            StorageCommands::Remove { key } => {
                StorageService::remove(context, &StorageKey::new(key))?.into()
            }
        };
        Ok(result)
    }
}
