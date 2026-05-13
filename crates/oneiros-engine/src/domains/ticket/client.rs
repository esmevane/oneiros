//! HTTP client for the ticket domain.

use crate::*;

/// Client scoped to ticket operations.
pub(crate) struct TicketClient<'a> {
    client: &'a Client,
}

impl<'a> TicketClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Issue a new ticket for the given actor and project.
    pub(crate) async fn issue(
        &self,
        issuance: &CreateTicket,
    ) -> Result<TicketResponse, ClientError> {
        self.client.post("/tickets", issuance).await
    }

    /// Retrieve a single ticket by key.
    #[cfg(test)]
    pub(crate) async fn get(&self, lookup: &GetTicket) -> Result<TicketResponse, ClientError> {
        let GetTicket::V1(lookup) = lookup;
        self.client.get(&format!("/tickets/{}", lookup.key)).await
    }

    /// List all tickets.
    pub(crate) async fn list(&self, listing: &ListTickets) -> Result<TicketResponse, ClientError> {
        let ListTickets::V1(listing) = listing;
        let query = format!(
            "limit={}&offset={}",
            listing.filters.limit, listing.filters.offset,
        );
        self.client.get(&format!("/tickets?{query}")).await
    }

    /// Validate a ticket token.
    pub(crate) async fn validate(
        &self,
        validation: &ValidateTicket,
    ) -> Result<TicketResponse, ClientError> {
        self.client.post("/tickets/validate", validation).await
    }
}
