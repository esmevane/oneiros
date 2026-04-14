use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub(crate) struct PressureCommands {
    #[command(flatten)]
    pub(crate) request: GetPressure,
}

impl PressureCommands {
    pub(crate) async fn execute(
        &self,
        client: &Client,
    ) -> Result<Rendered<Responses>, PressureError> {
        
        let pressure_client = PressureClient::new(client);

        let response = pressure_client.get(&self.request).await?;
        Ok(PressureView::new(response).render())
    }
}
