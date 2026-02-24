use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::{Outcome, Outcomes};
use std::path::PathBuf;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ServiceStatusOutcomes {
    #[outcome(message("Socket: {}", .0.display()))]
    SocketPath(PathBuf),
    #[outcome(
        message("No socket file found. Service has not been started."),
        level = "warn"
    )]
    NoSocket,
    #[outcome(message("Service is running."))]
    ServiceRunning,
    #[outcome(message("Service is not running: {0}"), level = "warn")]
    ServiceNotRunning(String),
}

#[derive(Clone, Args)]
pub struct Status;

impl Status {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<ServiceStatusOutcomes>, ServiceCommandError> {
        let mut outcomes = Outcomes::new();

        let socket_path = context.socket_path();

        outcomes.emit(ServiceStatusOutcomes::SocketPath(socket_path.clone()));

        if !socket_path.exists() {
            outcomes.emit(ServiceStatusOutcomes::NoSocket);
            return Ok(outcomes);
        }

        let client = Client::new(&socket_path);

        match client.health().await {
            Ok(()) => outcomes.emit(ServiceStatusOutcomes::ServiceRunning),
            Err(error) => {
                outcomes.emit(ServiceStatusOutcomes::ServiceNotRunning(error.to_string()))
            }
        }

        Ok(outcomes)
    }
}
