//! HTTP client for the ticket domain.

use crate::*;

/// Client scoped to ticket operations.
pub struct TicketClient<'a> {
    client: &'a Client,
}

impl<'a> TicketClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Issue a new ticket for the given actor and brain.
    pub async fn issue(
        &self,
        actor_id: &ActorId,
        brain_name: &BrainName,
    ) -> Result<TicketResponse, ClientError> {
        self.client
            .post(
                "/tickets/",
                &serde_json::json!({ "actor_id": actor_id, "brain_name": brain_name }),
            )
            .await
    }

    /// Retrieve a single ticket by ID.
    pub async fn get(&self, id: &TicketId) -> Result<TicketResponse, ClientError> {
        self.client.get(&format!("/tickets/{}", id)).await
    }

    /// List all tickets.
    pub async fn list(&self) -> Result<TicketResponse, ClientError> {
        self.client.get("/tickets/").await
    }

    /// Validate a ticket token.
    pub async fn validate(&self, token: &str) -> Result<TicketResponse, ClientError> {
        self.client
            .post("/tickets/validate", &serde_json::json!({ "token": token }))
            .await
    }
}
