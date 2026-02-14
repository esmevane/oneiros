mod outcomes;

use clap::Args;
use oneiros_outcomes::Outcomes;
use service_manager::*;

pub(crate) use outcomes::UninstallServiceOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct UninstallService;

impl UninstallService {
    pub(crate) async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<UninstallServiceOutcomes>, ServiceCommandError> {
        let mut outcomes = Outcomes::new();

        let label: ServiceLabel = context.service_label().parse()?;

        let mut manager = <dyn ServiceManager>::native()?;
        manager.set_level(ServiceLevel::User)?;

        // Best-effort stop before uninstall.
        let _ = manager.stop(ServiceStopCtx {
            label: label.clone(),
        });

        manager.uninstall(ServiceUninstallCtx { label })?;

        outcomes.emit(UninstallServiceOutcomes::ServiceUninstalled);

        Ok(outcomes)
    }
}
