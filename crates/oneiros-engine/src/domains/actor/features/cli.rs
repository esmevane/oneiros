use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

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
        context: &SystemContext,
        cmd: ActorCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            ActorCommands::Create { tenant_id, name } => ActorService::create(
                context,
                tenant_id.parse::<TenantId>()?,
                ActorName::new(name),
            )?
            .into(),
            ActorCommands::Get { id } => {
                ActorService::get(context, &id.parse::<ActorId>()?)?.into()
            }
            ActorCommands::List => ActorService::list(context)?.into(),
        };
        Ok(result)
    }
}
