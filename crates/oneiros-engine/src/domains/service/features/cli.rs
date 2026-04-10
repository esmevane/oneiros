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
    pub async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, ServiceError> {
        let response = match self {
            ServiceCommands::Install => ServiceService::install(config)?,
            ServiceCommands::Uninstall => ServiceService::uninstall(config)?,
            ServiceCommands::Start => ServiceService::start(config).await?,
            ServiceCommands::Stop => ServiceService::stop(config)?,
            ServiceCommands::Status => ServiceService::status(config).await,
            ServiceCommands::Run => ServiceService::run(config).await?,
        };

        Ok(Rendered::new(
            response.clone().into(),
            ServiceView::render(&response),
            String::new(),
        ))
    }
}
