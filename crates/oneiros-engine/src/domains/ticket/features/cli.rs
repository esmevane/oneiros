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
        context: &SystemContext,
        cmd: TicketCommands,
    ) -> Result<Responses, TicketError> {
        let result = match cmd {
            TicketCommands::Issue {
                actor_id,
                brain_name,
            } => TicketService::create(context, actor_id, BrainName::new(brain_name))?.into(),
            TicketCommands::Validate { id } => TicketService::validate(context, &id)?.into(),
            TicketCommands::List => TicketService::list(context)?.into(),
        };
        Ok(result)
    }
}
