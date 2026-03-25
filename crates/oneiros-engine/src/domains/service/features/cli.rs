use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub enum ServiceCommands {
    /// Install oneiros as a managed system service.
    Install,
    /// Uninstall the managed service.
    Uninstall,
    /// Start the managed service.
    Start,
    /// Stop the managed service.
    Stop,
    /// Check if the service is running.
    Status,
    /// Run the service directly (foreground).
    Run,
}

impl ServiceCommands {
    pub async fn execute(&self, engine: &Engine) -> Result<Rendered<Responses>, ServiceError> {
        let config = engine.service_config();
        let data_dir = engine.data_dir();

        let response = match self {
            ServiceCommands::Install => ServiceService::install(&config, data_dir)?,
            ServiceCommands::Uninstall => ServiceService::uninstall(&config)?,
            ServiceCommands::Start => ServiceService::start(&config).await?,
            ServiceCommands::Stop => ServiceService::stop(&config)?,
            ServiceCommands::Status => ServiceService::status(&config).await,
            ServiceCommands::Run => ServiceService::run(engine).await?,
        };

        Ok(Rendered::new(
            Response::new(response.clone().into()),
            response.to_string(),
            String::new(),
        ))
    }
}
