use oneiros_outcomes::Outcome;
use std::path::PathBuf;

#[derive(Clone, serde::Serialize, Outcome)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ServiceStatusOutcomes {
    #[outcome(message("Socket: {}", .0.display()))]
    SocketPath(PathBuf),
    #[outcome(
        message("No socket file found. Service has not been started."),
        level = "warn"
    )]
    NoSocket,
    #[outcome(message("Service is running."))]
    ServiceRunning,
    #[outcome(message("Service is not running: {0}"), level = "warn")]
    ServiceNotRunning(String),
}
