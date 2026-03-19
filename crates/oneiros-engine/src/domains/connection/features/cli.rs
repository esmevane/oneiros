use clap::Subcommand;

use crate::*;

pub struct ConnectionCli;

#[derive(Debug, Subcommand)]
pub enum ConnectionCommands {
    Create {
        nature: String,
        from_ref: String,
        to_ref: String,
    },
    Show {
        id: String,
    },
    List {
        #[arg(long)]
        nature: Option<String>,
        #[arg(long)]
        entity_ref: Option<String>,
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
                nature,
                from_ref,
                to_ref,
            } => serde_json::to_string_pretty(&ConnectionService::create(
                ctx,
                from_ref,
                to_ref,
                nature,
                String::new(),
            )?)?,
            ConnectionCommands::Show { id } => {
                serde_json::to_string_pretty(&ConnectionService::get(ctx, &id)?)?
            }
            ConnectionCommands::List { entity_ref, .. } => serde_json::to_string_pretty(
                &ConnectionService::list(ctx, entity_ref.as_deref())?,
            )?,
            ConnectionCommands::Remove { id } => {
                serde_json::to_string_pretty(&ConnectionService::remove(ctx, &id)?)?
            }
        };
        Ok(result)
    }
}
