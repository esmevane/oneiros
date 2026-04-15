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

        Ok(TicketView::new(response).render().map(Into::into))
    }
}
