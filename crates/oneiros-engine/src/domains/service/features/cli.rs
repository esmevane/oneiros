use std::net::SocketAddr;

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
    pub async fn execute(
        &self,
        engine: &Engine,
        service_addr: SocketAddr,
        data_dir: &std::path::Path,
    ) -> Result<Rendered<Responses>, ServiceError> {
        let response = match self {
            ServiceCommands::Install => ServiceService::install(&data_dir.to_path_buf())?,
            ServiceCommands::Uninstall => ServiceService::uninstall()?,
            ServiceCommands::Start => ServiceService::start(service_addr).await?,
            ServiceCommands::Stop => ServiceService::stop()?,
            ServiceCommands::Status => ServiceService::status(service_addr).await,
            ServiceCommands::Run => {
                return self.run_server(engine, service_addr).await;
            }
        };

        let prompt = match &response {
            ServiceResponse::ServiceInstalled(label) => {
                format!("Service installed as '{label}'.")
            }
            ServiceResponse::ServiceUninstalled => "Service uninstalled.".to_string(),
            ServiceResponse::ServiceStarted => "Service started.".to_string(),
            ServiceResponse::ServiceHealthy(addr) => {
                format!("Service started and healthy at {addr}.")
            }
            ServiceResponse::ServiceStopped => "Service stopped.".to_string(),
            ServiceResponse::ServiceRunning(addr) => {
                format!("Service is running at {addr}.")
            }
            ServiceResponse::ServiceNotRunning(reason) => {
                format!("Service is not running: {reason}")
            }
        };

        Ok(Rendered::new(
            Response::new(response.into()),
            prompt,
            String::new(),
        ))
    }

    /// Run the HTTP server directly in the foreground.
    async fn run_server(
        &self,
        engine: &Engine,
        addr: SocketAddr,
    ) -> Result<Rendered<Responses>, ServiceError> {
        let router = engine
            .project_router()
            .map_err(|e| ServiceError::Manager(e.to_string()))?;

        let listener = tokio::net::TcpListener::bind(addr).await?;

        // Note: this blocks until the server shuts down.
        axum::serve(listener, router.into_make_service())
            .await
            .map_err(|e| ServiceError::Io(e))?;

        let stopped = ServiceResponse::ServiceStopped;
        Ok(Rendered::new(
            Response::new(stopped.into()),
            "Service stopped.".to_string(),
            String::new(),
        ))
    }
}
