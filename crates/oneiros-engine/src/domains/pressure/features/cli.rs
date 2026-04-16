use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub struct PressureCommands {
    #[command(flatten)]
    pub request: GetPressure,
}

impl PressureCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, PressureError> {
        let client = context.client();
        let pressure_client = PressureClient::new(&client);

        let request = PressureRequest::GetPressure(self.request.clone());
        let response = pressure_client.get(&self.request).await?;
        Ok(PressureView::new(response, &request)
            .render()
            .map(Into::into))
    }
}
