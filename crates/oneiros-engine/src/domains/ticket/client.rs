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

    /// Issue a new ticket for the given actor and brain.
    pub(crate) async fn issue(&self, creation: &CreateTicket) -> Result<TicketResponse, ClientError> {
        self.client.post("/tickets", creation).await
    }

    /// Retrieve a single ticket by ID.
    pub(crate) async fn get(&self, id: &TicketId) -> Result<TicketResponse, ClientError> {
        self.client.get(&format!("/tickets/{}", id)).await
    }

    /// List all tickets.
    pub(crate) async fn list(&self, request: &ListTickets) -> Result<TicketResponse, ClientError> {
        let query = format!(
            "limit={}&offset={}",
            request.filters.limit, request.filters.offset,
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
