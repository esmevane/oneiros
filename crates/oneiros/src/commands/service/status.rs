use clap::Args;
use oneiros_client::{HttpClient, ServiceClient};
use oneiros_outcomes::Outcomes;

use super::error::ServiceCommandError;
use super::service_outcomes::StatusOutcomes;
use crate::*;

#[derive(Clone, Args)]
pub(crate) struct Status;

impl Status {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<StatusOutcomes>, ServiceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = HttpClient::new(context.socket_path());

        match client.health().await {
            Ok(()) => outcomes.emit(StatusOutcomes::ServiceRunning),
            Err(error) => outcomes.emit(StatusOutcomes::ServiceNotRunning(error.to_string())),
        }

        Ok(outcomes)
    }
}
