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
                let data = std::fs::read(&file)?;
                let content_type = if description.is_empty() {
                    Label::new("application/octet-stream")
                } else {
                    Label::new(description)
                };
                StorageService::upload(context, StorageName::new(key), content_type, data)?.into()
            }
            StorageCommands::Show { key } => {
                StorageService::show(context, &StorageName::new(key))?.into()
            }
            StorageCommands::List => StorageService::list(context)?.into(),
            StorageCommands::Remove { key } => {
                // Look up by name to get the ID, then remove by ID
                let entry = context
                    .with_db(|conn| StorageRepo::new(conn).get_by_name(&StorageName::new(&key)))?
                    .ok_or_else(|| StorageError::NameNotFound(StorageName::new(&key)))?;
                StorageService::remove(context, &entry.id)?.into()
            }
        };
        Ok(result)
    }
}
