mod outcomes;

use std::ffi::OsString;

use clap::Args;
use oneiros_outcomes::Outcomes;
use service_manager::*;

pub(crate) use outcomes::InstallServiceOutcomes;

use crate::*;

#[derive(Clone, Args)]
pub(crate) struct InstallService;

impl InstallService {
    pub(crate) async fn run(
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
