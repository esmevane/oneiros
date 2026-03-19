use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

pub struct TicketCli;

#[derive(Debug, Subcommand)]
pub enum TicketCommands {
    Issue {
        #[arg(long)]
        actor_id: ActorId,
        #[arg(long)]
        brain_name: String,
    },
    Validate {
        id: String,
    },
    List,
}

impl TicketCli {
    pub fn execute(
        ctx: &SystemContext,
        cmd: TicketCommands,
    ) -> Result<Responses, Box<dyn std::error::Error>> {
        let result = match cmd {
            TicketCommands::Issue {
                actor_id,
                brain_name,
            } => TicketService::create(ctx, actor_id, brain_name)?.into(),
            TicketCommands::Validate { id } => TicketService::validate(ctx, &id)?.into(),
            TicketCommands::List => TicketService::list(ctx)?.into(),
        };
        Ok(result)
    }
}
