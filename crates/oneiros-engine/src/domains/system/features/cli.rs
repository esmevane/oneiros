use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SystemCommands {
    Init(InitSystem),
}

impl SystemCommands {
    pub async fn execute(&self, context: HostLog) -> Result<Rendered<Responses>, SystemError> {
        let client = context.client();
        let system_client = SystemClient::new(&client);

        let response = match self {
            SystemCommands::Init(initialization) => system_client.init(initialization).await?,
        };

        Ok(SystemView::new(response).render().map(Into::into))
    }
}
