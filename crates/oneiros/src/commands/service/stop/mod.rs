mod outcomes;

use clap::Args;
use oneiros_outcomes::Outcomes;
use service_manager::*;

pub(crate) use outcomes::StopServiceOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct StopService;

impl StopService {
    pub(crate) async fn run(
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
