mod error;
mod outcomes;

use clap::Args;
use oneiros_client::Client;
use oneiros_db::Database;
use oneiros_outcomes::Outcomes;
use std::path::PathBuf;

pub(crate) use error::*;
pub(crate) use outcomes::DoctorOutcomes;

use crate::*;

#[derive(Args, Clone)]
pub(crate) struct DoctorOp;

impl DoctorOp {
    pub(crate) async fn run(
        &self,
        context: Context,
    ) -> Result<Outcomes<DoctorOutcomes>, CheckupError> {
        let mut checks = Outcomes::new();

        if let Some(name) = context.project_name() {
            let root = context
                .project_root()
                .map(PathBuf::from)
                .unwrap_or_default();
            checks.emit(DoctorOutcomes::ProjectDetected(name.to_string(), root));
        } else {
            checks.emit(DoctorOutcomes::NoProjectDetected);
        }

        if context.is_initialized() {
            checks.emit(DoctorOutcomes::Initialized);
        } else {
            checks.emit(DoctorOutcomes::NotInitialized);
        }

        match Database::open(context.db_path()) {
            Ok(store) => {
                checks.emit(DoctorOutcomes::DatabaseOk(context.db_path()));

                match store.event_count() {
                    Ok(count) => checks.emit(DoctorOutcomes::EventLogReady(count)),
                    Err(error) => checks.emit(DoctorOutcomes::NoEventLog(error.to_string())),
                };
            }
            Err(error) => {
                checks.emit(DoctorOutcomes::NoDatabaseFound(
                    context.db_path(),
                    error.to_string(),
                ));
            }
        }

        if context.config_path().exists() {
            checks.emit(DoctorOutcomes::ConfigOk(context.config_path()));
        } else {
            checks.emit(DoctorOutcomes::NoConfigFound(context.config_path()));
        }

        let client = Client::new(context.socket_path());

        match client.health().await {
            Ok(()) => checks.emit(DoctorOutcomes::ServiceRunning),
            Err(error) => checks.emit(DoctorOutcomes::ServiceNotRunning(error.to_string())),
        }

        Ok(checks)
    }
}
