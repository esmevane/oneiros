use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum TicketCommands {
    Issue(CreateTicket),
    Validate(ValidateTicket),
    List(ListTickets),
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
            TicketCommands::List(list) => ticket_client.list(list).await?,
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
            TicketResponse::Listed(listed) => {
                let mut out = format!("{} found of {} total.\n\n", listed.len(), listed.total);
                for ticket in &listed.items {
                    out.push_str(&format!(
                        "  {} ({})\n\n",
                        ticket.brain_name, ticket.actor_id,
                    ));
                }
                out
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
