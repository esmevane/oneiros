use clap::Subcommand;

use crate::contexts::SystemContext;
use crate::*;

#[derive(Debug, Subcommand)]
pub enum TicketCommands {
    Issue(CreateTicket),
    Validate(ValidateTicket),
    List,
}

impl TicketCommands {
    pub async fn execute(
        &self,
        context: &SystemContext,
    ) -> Result<Rendered<Responses>, TicketError> {
        let response = match self {
            TicketCommands::Issue(create) => {
                TicketService::create(context, create.actor_id, create.brain_name.clone()).await?
            }
            TicketCommands::Validate(validate) => {
                TicketService::validate(context, validate.token.as_str()).await?
            }
            TicketCommands::List => TicketService::list(context).await?,
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
