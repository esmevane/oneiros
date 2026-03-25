use std::fmt;

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

impl fmt::Display for ServiceResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceInstalled(label) => write!(f, "Service installed as '{label}'."),
            Self::ServiceUninstalled => write!(f, "Service uninstalled."),
            Self::ServiceStarted => write!(f, "Service started."),
            Self::ServiceHealthy(addr) => {
                write!(f, "Service started and healthy at {addr}.")
            }
            Self::ServiceStopped => write!(f, "Service stopped."),
            Self::ServiceRunning(addr) => write!(f, "Service is running at {addr}."),
            Self::ServiceNotRunning(reason) => {
                write!(f, "Service is not running: {reason}")
            }
        }
    }
}
