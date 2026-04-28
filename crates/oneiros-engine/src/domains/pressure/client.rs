//! HTTP client for the pressure domain.

use crate::*;

/// Client scoped to pressure operations.
pub struct PressureClient<'a> {
    client: &'a Client,
}

impl<'a> PressureClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieve pressure readings for a specific agent.
    pub async fn get(&self, request: &GetPressure) -> Result<PressureResponse, ClientError> {
        let details = request.current()?;
        self.client
            .get(&format!("/pressures/{}", details.agent))
            .await
    }

    /// List pressure readings for all agents.
    pub async fn list(&self) -> Result<PressureResponse, ClientError> {
        self.client.get("/pressures").await
    }
}
