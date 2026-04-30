use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub struct PressureCommands {
    #[command(flatten)]
    pub command: GetPressure,
}

impl PressureCommands {
    pub async fn execute(
        &self,
        context: &ProjectLog,
    ) -> Result<Rendered<Responses>, PressureError> {
        let client = context.client();
        let pressure_client = PressureClient::new(&client);

        let request = PressureRequest::GetPressure(self.command.clone());
        let response = pressure_client.get(&self.command).await?;
        Ok(PressurePresenter::new(response, &request)
            .render()
            .map(Into::into))
    }
}
