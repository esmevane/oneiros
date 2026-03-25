use serde::{Deserialize, Serialize};

/// All responses the service domain can produce.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ServiceResponse {
    ServiceInstalled(String),
    ServiceUninstalled,
    ServiceStarted,
    ServiceHealthy(String),
    ServiceStopped,
    ServiceRunning(String),
    ServiceNotRunning(String),
}
