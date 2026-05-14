use clap::Subcommand;

use crate::*;

#[derive(Debug, Subcommand)]
pub(crate) enum HostCommands {
    Init(InitHost),
    /// Install oneiros as a managed host service.
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

impl HostCommands {
    pub(crate) async fn execute(&self, config: &Config) -> Result<Rendered<Responses>, HostError> {
        let response = match self {
            HostCommands::Init(initialization) => {
                let client = Client::from_config(config)?;
                let bytes = initialization.execute_request(&client).await?;
                serde_json::from_slice::<HostResponse>(&bytes)?
            }
            HostCommands::Install => HostService::install(config)?,
            HostCommands::Uninstall => HostService::uninstall(config)?,
            HostCommands::Start => HostService::start(config).await?,
            HostCommands::Stop => HostService::stop(config)?,
            HostCommands::Status => HostService::status(config).await,
            HostCommands::Run => HostService::run(config).await?,
        };

        Ok(HostView::new(response).render().map(Into::into))
    }
}
