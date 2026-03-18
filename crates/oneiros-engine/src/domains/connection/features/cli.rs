use clap::Subcommand;

use crate::*;

pub struct ConnectionCli;

#[derive(Debug, Subcommand)]
pub enum ConnectionCommands {
    Create {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        nature: String,
        description: String,
    },
    Get {
        id: String,
    },
    List {
        #[arg(long)]
        entity: Option<String>,
    },
    Remove {
        id: String,
    },
}

impl ConnectionCli {
    pub fn execute(
        ctx: &ProjectContext,
        cmd: ConnectionCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            ConnectionCommands::Create {
                from,
                to,
                nature,
                description,
            } => serde_json::to_string_pretty(&ConnectionService::create(
                ctx,
                from,
                to,
                nature,
                description,
            )?)?,
            ConnectionCommands::Get { id } => {
                serde_json::to_string_pretty(&ConnectionService::get(ctx, &id)?)?
            }
            ConnectionCommands::List { entity } => {
                serde_json::to_string_pretty(&ConnectionService::list(ctx, entity.as_deref())?)?
            }
            ConnectionCommands::Remove { id } => {
                serde_json::to_string_pretty(&ConnectionService::remove(ctx, &id)?)?
            }
        };
        Ok(result)
    }
}
