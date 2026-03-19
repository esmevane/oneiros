use std::path::PathBuf;

use clap::Subcommand;

use crate::*;

pub struct StorageCli;

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

impl StorageCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: StorageCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            StorageCommands::Set {
                key,
                file,
                description,
            } => {
                let data = std::fs::read(&file)?;
                let content_type = if description.is_empty() {
                    "application/octet-stream".to_string()
                } else {
                    description
                };
                StorageService::upload(ctx, key, content_type, data)?.into()
            }
            StorageCommands::Show { key } => StorageService::show(ctx, &key)?.into(),
            StorageCommands::List => StorageService::list(ctx)?.into(),
            StorageCommands::Remove { key } => {
                // Look up by name to get the ID, then remove by ID
                let entry = ctx
                    .with_db(|conn| StorageRepo::new(conn).get_by_name(&key))
                    .map_err(|e| format!("database error: {e}"))?
                    .ok_or_else(|| format!("storage entry '{key}' not found"))?;
                StorageService::remove(ctx, &entry.id.to_string())?.into()
            }
        };
        Ok(result)
    }
}
