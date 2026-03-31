use clap::Subcommand;

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
        let client = context.client();
        let ticket_client = TicketClient::new(&client);

        let response = match self {
            TicketCommands::Issue(create) => ticket_client.issue(create).await?,
            TicketCommands::Validate(validate) => ticket_client.validate(validate).await?,
            TicketCommands::List => ticket_client.list().await?,
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
