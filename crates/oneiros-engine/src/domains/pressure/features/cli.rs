use clap::Args;

use crate::*;

#[derive(Debug, Args)]
pub struct PressureCommands {
    pub name: String,
}

impl PressureCommands {
    pub async fn execute(&self, context: &ProjectContext) -> Result<Responses, PressureError> {
        let client = context.client();
        let pressure_client = PressureClient::new(&client);

        let result = pressure_client
            .get(&AgentName::new(&self.name))
            .await?
            .into();
        Ok(result)
    }
}
