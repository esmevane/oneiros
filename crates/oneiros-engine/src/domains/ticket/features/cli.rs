use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

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

impl TicketCommands {
    pub fn execute(&self, context: &SystemContext) -> Result<Rendered<Responses>, TicketError> {
        let response = match self {
            TicketCommands::Issue {
                actor_id,
                brain_name,
            } => TicketService::create(context, actor_id.clone(), BrainName::new(brain_name))?,
            TicketCommands::Validate { id } => TicketService::validate(context, id)?,
            TicketCommands::List => TicketService::list(context)?,
        };

        let prompt = match &response {
            TicketResponse::Created(ticket) => {
                format!("Ticket issued for brain '{}'.", ticket.brain_name)
            }
            TicketResponse::Found(ticket) => {
                format!("Ticket for brain '{}'.", ticket.brain_name)
            }
            TicketResponse::Validated(ticket) => {
                format!("Ticket for brain '{}' is valid.", ticket.brain_name)
            }
            TicketResponse::Listed(tickets) => format!("{} ticket(s) found.", tickets.len()),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
