use clap::Subcommand;

use crate::*;

/// CLI subcommands for the ticket domain. Each variant carries a versioned
/// protocol request directly — clap derives parsing through the wrapper's
/// `Args` impl, which delegates to the latest version's struct.
#[derive(Debug, Subcommand)]
pub(crate) enum TicketCommands {
    Issue(CreateTicket),
    Revoke(RevokeTicket),
    Validate(ValidateTicket),
    List(ListTickets),
}

impl TicketCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, TicketError> {
        let client = Client::from_config(config)?;

        let bytes = match self {
            Self::Issue(issuance) => issuance.execute_request(&client).await?,
            Self::Revoke(revoke) => revoke.execute_request(&client).await?,
            Self::Validate(validation) => validation.execute_request(&client).await?,
            Self::List(listing) => listing.execute_request(&client).await?,
        };

        let response: TicketResponse = serde_json::from_slice(&bytes)?;
        Ok(TicketView::new(response).render().map(Into::into))
    }
}
