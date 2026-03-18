use clap::Subcommand;

use crate::*;
use crate::contexts::SystemContext;

pub struct ActorCli;

#[derive(Debug, Subcommand)]
pub enum ActorCommands {
    Create {
        #[arg(long)]
        tenant_id: String,
        name: String,
    },
    Get {
        id: String,
    },
    List,
}

impl ActorCli {
    pub fn execute(
        ctx: &SystemContext,
        cmd: ActorCommands,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match cmd {
            ActorCommands::Create { tenant_id, name } => {
                serde_json::to_string_pretty(&ActorService::create(ctx, tenant_id, name)?)?
            }
            ActorCommands::Get { id } => {
                serde_json::to_string_pretty(&ActorService::get(ctx, &id)?)?
            }
            ActorCommands::List => {
                serde_json::to_string_pretty(&ActorService::list(ctx)?)?
            }
        };
        Ok(result)
    }
}
