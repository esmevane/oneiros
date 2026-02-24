use clap::Args;
use oneiros_client::Client;
use oneiros_outcomes::{Outcome, Outcomes};
use service_manager::*;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StartServiceOutcomes {
    #[outcome(message("Service started."))]
    Started,
    #[outcome(message("Service is running."))]
    Healthy,
    #[outcome(
        message("Service started but health check failed: {0}"),
        level = "warn"
    )]
    StartedButUnhealthy(String),
}

#[derive(Clone, Args)]
pub struct StartService;

impl StartService {
    pub async fn run(
        &self,
        context: &Context,
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
