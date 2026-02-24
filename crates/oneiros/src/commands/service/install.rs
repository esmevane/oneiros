use std::ffi::OsString;

use clap::Args;
use oneiros_outcomes::{Outcome, Outcomes};
use service_manager::*;

use crate::*;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum InstallServiceOutcomes {
    #[outcome(message("Service installed as '{0}'."))]
    ServiceInstalled(String),
}

#[derive(Clone, Args)]
pub struct InstallService;

impl InstallService {
    pub async fn run(
        &self,
        context: &Context,
    ) -> Result<Outcomes<InstallServiceOutcomes>, ServiceCommandError> {
        let mut outcomes = Outcomes::new();

        let label = context.service_label();

        let mut manager = <dyn ServiceManager>::native()?;
        manager.set_level(ServiceLevel::User)?;

        context.files().ensure_dir(context.log_dir())?;

        manager.install(ServiceInstallCtx {
            label: label.parse()?,
            program: context.current_exe()?,
            args: vec![OsString::from("service"), OsString::from("run")],
            contents: None,
            username: None,
            working_directory: Some(context.data_dir.clone()),
            environment: None,
            autostart: true,
            restart_policy: RestartPolicy::OnFailure {
                delay_secs: Some(5),
            },
        })?;

        outcomes.emit(InstallServiceOutcomes::ServiceInstalled(label));

        Ok(outcomes)
    }
}
