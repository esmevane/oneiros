use oneiros_outcomes::Outcome;
use std::path::PathBuf;

#[derive(Clone, Outcome)]
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
