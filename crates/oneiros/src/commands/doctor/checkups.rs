use std::path::PathBuf;

use crate::*;

#[derive(Clone)]
pub(crate) enum Checkups {
    ProjectDetected(String, PathBuf),
    NoProjectDetected,
    Initialized,
    NotInitialized,
    DatabaseOk(PathBuf),
    NoDatabaseFound(PathBuf, String),
    EventLogReady(usize),
    NoEventLog(String),
    ConfigOk(PathBuf),
    NoConfigFound(PathBuf),
    NoContextAvailable,
}

impl Reportable for Checkups {
    fn report(&self) {
        match self {
            Checkups::NoContextAvailable => {
                tracing::error!(
                    "No context available. Please run this command within a valid project directory."
                );
            }
            Checkups::ProjectDetected(name, root) => {
                tracing::info!("Project '{}' detected at '{}'.", name, root.display());
            }
            Checkups::NoProjectDetected => {
                tracing::warn!("No project detected.");
            }
            Checkups::Initialized => {
                tracing::info!("System is initialized.");
            }
            Checkups::NotInitialized => {
                tracing::warn!("System is not initialized.");
            }
            Checkups::DatabaseOk(path) => {
                tracing::info!("Database found at '{}'.", path.display());
            }
            Checkups::NoDatabaseFound(path, error) => {
                tracing::warn!("Database not found at '{}': {}", path.display(), error);
            }
            Checkups::EventLogReady(count) => {
                tracing::info!("Event log is ready with {} events.", count);
            }
            Checkups::NoEventLog(error) => {
                tracing::warn!("Event log error: {}", error);
            }
            Checkups::ConfigOk(path) => {
                tracing::info!("Config file found at '{}'.", path.display());
            }
            Checkups::NoConfigFound(path) => {
                tracing::warn!("Config file not found at '{}'.", path.display());
            }
        }
    }
}
