use std::path::PathBuf;

use clap::Args;
use oneiros_db::Database;

use crate::*;

#[derive(Args, Clone)]
pub(crate) struct Checkup;

impl Checkup {
    pub(crate) async fn run(
        &self,
        context: Option<Context>,
    ) -> Result<Vec<Checkups>, CheckupError> {
        let Some(context) = context else {
            return Ok(vec![Checkups::NoContextAvailable]);
        };

        let mut checks = vec![];

        // Report detected project
        if let Some(name) = context.project_name() {
            let root = context
                .project_root()
                .map(PathBuf::from)
                .unwrap_or_default();
            checks.push(Checkups::ProjectDetected(name.to_string(), root));
        } else {
            checks.push(Checkups::NoProjectDetected);
        }

        if context.is_initialized() {
            checks.push(Checkups::Initialized);
        } else {
            checks.push(Checkups::NotInitialized);
        }

        match Database::open(context.db_path()) {
            Ok(store) => {
                checks.push(Checkups::DatabaseOk(context.db_path()));

                match store.event_count() {
                    Ok(count) => checks.push(Checkups::EventLogReady(count)),
                    Err(error) => checks.push(Checkups::NoEventLog(error.to_string())),
                };
            }
            Err(error) => {
                checks.push(Checkups::NoDatabaseFound(
                    context.db_path(),
                    error.to_string(),
                ));
            }
        }

        if context.config_path().exists() {
            checks.push(Checkups::ConfigOk(context.config_path()));
        } else {
            checks.push(Checkups::NoConfigFound(context.config_path()));
        }

        Ok(checks)
    }
}
