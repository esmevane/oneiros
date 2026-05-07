use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub(crate) struct PressureCommands {
    #[command(flatten)]
    pub(crate) command: GetPressure,
}

impl PressureCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, PressureError> {
        let client = Client::new(config.base_url());
        let pressure_client = PressureClient::new(&client);

        let request = PressureRequest::GetPressure(self.command.clone());
        let response = pressure_client.get(&self.command).await?;
        Ok(PressurePresenter::new(response, &request)
            .render()
            .map(Into::into))
    }
}
