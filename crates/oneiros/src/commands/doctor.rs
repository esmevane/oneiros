use clap::Args;
use oneiros_client::Client;
use oneiros_db::Database;
use oneiros_outcomes::{Outcome, Outcomes};
use std::path::PathBuf;

use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum CheckupError {}

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum DoctorOutcomes {
    #[outcome(message("Project '{0}' detected at '{}'.", .1.display()))]
    ProjectDetected(String, PathBuf),
    #[outcome(message("No project detected."), level = "warn")]
    NoProjectDetected,
    #[outcome(message("System is initialized."))]
    Initialized,
    #[outcome(message("System is not initialized."), level = "warn")]
    NotInitialized,
    #[outcome(message("Database found at '{}'.", .0.display()))]
    DatabaseOk(PathBuf),
    #[outcome(message("Database not found at '{}': {1}", .0.display()), level = "warn")]
    NoDatabaseFound(PathBuf, String),
    #[outcome(message("Event log is ready with {0} events."))]
    EventLogReady(usize),
    #[outcome(message("Event log error: {0}"), level = "warn")]
    NoEventLog(String),
    #[outcome(message("Config file found at '{}'.", .0.display()))]
    ConfigOk(PathBuf),
    #[outcome(message("Config file not found at '{}'.", .0.display()), level = "warn")]
    NoConfigFound(PathBuf),
    #[outcome(message("Service is running."))]
    ServiceRunning,
    #[outcome(message("Service is not running: {0}"), level = "warn")]
    ServiceNotRunning(String),
}

#[derive(Args, Clone)]
pub struct DoctorOp;

impl DoctorOp {
    pub async fn run(&self, context: &Context) -> Result<Outcomes<DoctorOutcomes>, CheckupError> {
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
