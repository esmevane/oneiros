use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum TicketCommands {
    Issue(CreateTicket),
    Validate(ValidateTicket),
    List(ListTickets),
}

impl TicketCommands {
    pub(crate) async fn execute(
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
            TicketResponse::Created(ticket) => TicketView::confirmed("issued", ticket).to_string(),
            TicketResponse::Found(ticket) => TicketView::detail(ticket).to_string(),
            TicketResponse::Validated(ticket) => {
                TicketView::confirmed("validated", ticket).to_string()
            }
            TicketResponse::Listed(listed) => {
                let table = TicketView::table(listed);
                format!(
                    "{}\n\n{table}",
                    format_args!("{} of {} total", listed.len(), listed.total).muted(),
                )
            }
        };

        Ok(Rendered::new(response.into(), prompt, String::new()))
    }
}
