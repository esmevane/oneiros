mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::Outcomes;
use service_manager::*;

pub(crate) use outcomes::StartServiceOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct StartService;

impl StartService {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<StartServiceOutcomes>, ServiceCommandError> {
        let mut outcomes = Outcomes::new();

        let label: ServiceLabel = context.service_label().parse()?;

        let mut manager = <dyn ServiceManager>::native()?;
        manager.set_level(ServiceLevel::User)?;

        manager.start(ServiceStartCtx { label })?;

        outcomes.emit(StartServiceOutcomes::Started);

        // Brief health check with backoff to confirm the service came up.
        let client = Client::new(context.socket_path());
        let delays = context.health_check_delays();

        for delay in delays {
            tokio::time::sleep(*delay).await;

            if client.health().await.is_ok() {
                outcomes.emit(StartServiceOutcomes::Healthy);
                return Ok(outcomes);
            }
        }

        let total: std::time::Duration = delays.iter().sum();
        outcomes.emit(StartServiceOutcomes::StartedButUnhealthy(format!(
            "health check did not succeed within {total:.0?}"
        )));

        Ok(outcomes)
    }
}
