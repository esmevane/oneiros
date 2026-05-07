use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum SystemCommands {
    Init(InitSystem),
}

impl SystemCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, SystemError> {
        let client = Client::new(config.base_url());
        let system_client = SystemClient::new(&client);

        let response = match self {
            SystemCommands::Init(initialization) => system_client.init(initialization).await?,
        };

        Ok(SystemView::new(response).render().map(Into::into))
    }
}
