use clap::Subcommand;

use crate::*;

/// CLI subcommands for the ticket domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct.
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
            Self::Issue(issuance) => ticket_client.issue(issuance).await?,
            Self::Validate(validation) => ticket_client.validate(validation).await?,
            Self::List(listing) => ticket_client.list(listing).await?,
        };

        Ok(TicketView::new(response).render().map(Into::into))
    }
}
