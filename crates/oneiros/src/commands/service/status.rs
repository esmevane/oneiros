use std::net::SocketAddr;

use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ServiceStatusOutcomes {
    #[outcome(message("Endpoint: {0}"))]
    Endpoint(SocketAddr),
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

        let addr = context.config().service_addr();
        outcomes.emit(ServiceStatusOutcomes::Endpoint(addr));

        let client = context.client();

        match client.health().await {
            Ok(()) => outcomes.emit(ServiceStatusOutcomes::ServiceRunning),
            Err(error) => {
                outcomes.emit(ServiceStatusOutcomes::ServiceNotRunning(error.to_string()))
            }
        }

        Ok(outcomes)
    }
}
