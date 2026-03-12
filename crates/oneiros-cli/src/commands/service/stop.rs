use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};
use service_manager::*;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum StopServiceOutcomes {
    #[outcome(message("Service stopped."))]
    ServiceStopped,
}

#[derive(Clone, Args)]
pub struct StopService;

impl StopService {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<StopServiceOutcomes>, ServiceCommandError> {
        let mut outcomes = Outcomes::new();

        let label: ServiceLabel = context.service_label().parse()?;

        let mut manager = <dyn ServiceManager>::native()?;
        manager.set_level(ServiceLevel::User)?;

        manager.stop(ServiceStopCtx { label })?;

        outcomes.emit(StopServiceOutcomes::ServiceStopped);

        Ok(outcomes)
    }
}
