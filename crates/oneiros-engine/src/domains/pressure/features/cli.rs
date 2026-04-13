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
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, PressureError> {
        let client = context.client();
        let pressure_client = PressureClient::new(&client);

        let response = pressure_client.get(&self.request).await?;
        Ok(PressureView::new(response).render())
    }
}
