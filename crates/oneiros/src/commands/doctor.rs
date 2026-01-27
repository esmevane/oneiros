use clap::Args;

use crate::{Context, Database};

use checks::Checks;

#[derive(Args, Clone)]
pub(crate) struct Doctor;

impl Doctor {
    pub(crate) async fn run(&self, context: Option<Context>) {
        let Some(context) = context else {
            tracing::error!(
                "No context available. Please run this command within a valid project directory."
            );
            return;
        };

        let mut checks = vec![];

        if context.is_initialized() {
            checks.push(Checks::Initialized);
        } else {
            checks.push(Checks::NotInitialized);
        }

        match Database::open(context.db_path()) {
            Ok(store) => {
                checks.push(Checks::DatabaseOk(context.db_path()));

                match store.event_count() {
                    Ok(count) => checks.push(Checks::EventLogReady(count)),
                    Err(error) => checks.push(Checks::NoEventLog(error.to_string())),
                };
            }
            Err(error) => {
                checks.push(Checks::NoDatabaseFound(
                    context.db_path(),
                    error.to_string(),
                ));
            }
        }

        if context.config_path().exists() {
            checks.push(Checks::ConfigOk(context.config_path()));
        } else {
            checks.push(Checks::NoConfigFound(context.config_path()));
        }

        for checks in checks {
            checks.report();
        }
    }
}

mod checks {
    use std::path::PathBuf;

    pub(crate) enum Checks {
        Initialized,
        NotInitialized,
        DatabaseOk(PathBuf),
        NoDatabaseFound(PathBuf, String),
        EventLogReady(usize),
        NoEventLog(String),
        ConfigOk(std::path::PathBuf),
        NoConfigFound(std::path::PathBuf),
    }

    impl Checks {
        pub(crate) fn report(&self) {
            match self {
                Checks::Initialized => {
                    tracing::info!("Project is initialized.");
                }
                Checks::NotInitialized => {
                    tracing::warn!("Project is not initialized.");
                }
                Checks::DatabaseOk(path) => {
                    tracing::info!("Database found at '{}'.", path.display());
                }
                Checks::NoDatabaseFound(path, error) => {
                    tracing::warn!("Database not found at '{}': {}", path.display(), error);
                }
                Checks::EventLogReady(count) => {
                    tracing::info!("Event log is ready with {} events.", count);
                }
                Checks::NoEventLog(error) => {
                    tracing::warn!("Event log error: {}", error);
                }
                Checks::ConfigOk(path) => {
                    tracing::info!("Config file found at '{}'.", path.display());
                }
                Checks::NoConfigFound(path) => {
                    tracing::warn!("Config file not found at '{}'.", path.display());
                }
            }
        }
    }
}
