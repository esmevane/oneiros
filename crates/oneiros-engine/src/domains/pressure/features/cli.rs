use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub(crate) struct PressureCommands {
    #[command(flatten)]
    pub(crate) command: GetPressure,
}

impl PressureCommands {
    pub(crate) async fn execute(
        &self,
        config: &Config,
    ) -> Result<Rendered<Responses>, PressureError> {
        let client = Client::from_config(config)?;

        let bytes = self.command.execute_request(&client).await?;
        let response: PressureResponse = serde_json::from_slice(&bytes)?;
        let request = PressureRequest::GetPressure(self.command.clone());
        Ok(PressureView::new(response, &request)
            .render()
            .map(Into::into))
    }
}
