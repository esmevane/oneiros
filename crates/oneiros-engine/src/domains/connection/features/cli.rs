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
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            ConnectionCommands::Create {
                nature,
                from_ref,
                to_ref,
            } => ConnectionService::create(ctx, from_ref, to_ref, nature, String::new())?.into(),
            ConnectionCommands::Show { id } => ConnectionService::get(ctx, &id)?.into(),
            ConnectionCommands::List { entity_ref, .. } => {
                ConnectionService::list(ctx, entity_ref.as_deref())?.into()
            }
            ConnectionCommands::Remove { id } => ConnectionService::remove(ctx, &id)?.into(),
        };
        Ok(result)
    }
}
