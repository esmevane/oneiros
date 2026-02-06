use std::path::PathBuf;

use clap::Args;
use oneiros_db::Database;
use oneiros_outcomes::Outcomes;

use crate::*;

#[derive(Args, Clone)]
pub(crate) struct Checkup;

impl Checkup {
    pub(crate) async fn run(
        &self,
        context: Option<Context>,
    ) -> Result<Outcomes<Checkups>, CheckupError> {
        let mut checks = Outcomes::new();

        let Some(context) = context else {
            checks.emit(Checkups::NoContextAvailable);
            return Ok(checks);
        };

        if let Some(name) = context.project_name() {
            let root = context
                .project_root()
                .map(PathBuf::from)
                .unwrap_or_default();
            checks.emit(Checkups::ProjectDetected(name.to_string(), root));
        } else {
            checks.emit(Checkups::NoProjectDetected);
        }

        if context.is_initialized() {
            checks.emit(Checkups::Initialized);
        } else {
            checks.emit(Checkups::NotInitialized);
        }

        match Database::open(context.db_path()) {
            Ok(store) => {
                checks.emit(Checkups::DatabaseOk(context.db_path()));

                match store.event_count() {
                    Ok(count) => checks.emit(Checkups::EventLogReady(count)),
                    Err(error) => checks.emit(Checkups::NoEventLog(error.to_string())),
                };
            }
            Err(error) => {
                checks.emit(Checkups::NoDatabaseFound(
                    context.db_path(),
                    error.to_string(),
                ));
            }
        }

        if context.config_path().exists() {
            checks.emit(Checkups::ConfigOk(context.config_path()));
        } else {
            checks.emit(Checkups::NoConfigFound(context.config_path()));
        }

        Ok(checks)
    }
}
