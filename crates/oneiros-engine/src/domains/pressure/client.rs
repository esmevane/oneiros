//! HTTP client for the pressure domain.

use crate::*;

/// Client scoped to pressure operations.
pub(crate) struct PressureClient<'a> {
    client: &'a Client,
}

impl<'a> PressureClient<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieve pressure readings for a specific agent.
    pub(crate) async fn get(&self, request: &GetPressure) -> Result<PressureResponse, ClientError> {
        let details = request.current()?;
        self.client
            .get(&format!("/pressures/{}", details.agent))
            .await
    }
}
