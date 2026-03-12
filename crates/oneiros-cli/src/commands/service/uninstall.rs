use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};
use service_manager::*;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum UninstallServiceOutcomes {
    #[outcome(message("Service uninstalled."))]
    ServiceUninstalled,
}

#[derive(Clone, Args)]
pub struct UninstallService;

impl UninstallService {
    pub async fn run(
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
