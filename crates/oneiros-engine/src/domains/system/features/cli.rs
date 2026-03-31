use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum SystemCommands {
    Init(InitSystem),
}

impl SystemCommands {
    pub async fn execute(
        &self,
        context: SystemContext,
    ) -> Result<Rendered<Responses>, SystemError> {
        let response = match self {
            SystemCommands::Init(init) => SystemService::init(&context, init).await?,
        };

        let prompt = match &response {
            SystemResponse::SystemInitialized(name) => {
                format!("System initialized for '{name}'.")
            }
            SystemResponse::HostAlreadyInitialized => "System already initialized.".to_string(),
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }
}
