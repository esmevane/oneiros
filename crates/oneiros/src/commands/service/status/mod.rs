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
