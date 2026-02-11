mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;

pub(crate) use outcomes::ServiceStatusOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct Status;

impl Status {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<ServiceStatusOutcomes>, ServiceCommandError> {
        let mut outcomes = Outcomes::new();

        let client = Client::new(context.socket_path());

        match client.health().await {
            Ok(()) => outcomes.emit(ServiceStatusOutcomes::ServiceRunning),
            Err(error) => {
                outcomes.emit(ServiceStatusOutcomes::ServiceNotRunning(error.to_string()))
            }
        }

        Ok(outcomes)
    }
}
