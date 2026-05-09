use clap::Subcommand;

use crate::*;

/// CLI subcommands for the ticket domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct.
#[derive(Debug, Subcommand)]
pub(crate) enum TicketCommands {
    Issue(CreateTicket),
    Validate(ValidateTicket),
    List(ListTickets),
}

impl TicketCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, TicketError> {
        let client = Client::from_config(config)?;
        let ticket_client = TicketClient::new(&client);

        let response = match self {
            Self::Issue(issuance) => ticket_client.issue(issuance).await?,
            Self::Validate(validation) => ticket_client.validate(validation).await?,
            Self::List(listing) => ticket_client.list(listing).await?,
        };

        Ok(TicketView::new(response).render().map(Into::into))
    }
}
