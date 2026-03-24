use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub struct PressureCommands {
    pub name: String,
}

impl PressureCommands {
    pub async fn execute(
        &self,
        context: &ProjectContext,
    ) -> Result<Rendered<Responses>, PressureError> {
        let client = context.client();
        let pressure_client = PressureClient::new(&client);

        let response = pressure_client.get(&AgentName::new(&self.name)).await?;
        Ok(PressurePresenter::new(response).render())
    }
}
