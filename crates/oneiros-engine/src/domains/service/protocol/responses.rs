use std::fmt;

use kinded::Kinded;
use serde::{Deserialize, Serialize};

/// The name under which the service is registered with the OS service manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct ServiceName(pub(crate) String);

impl ServiceName {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl fmt::Display for ServiceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// The network address at which the service is reachable.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct ServiceAddress(pub(crate) String);

impl ServiceAddress {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl fmt::Display for ServiceAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A human-readable explanation for why the service is not running.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct ServiceReason(pub(crate) String);

impl ServiceReason {
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl fmt::Display for ServiceReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// All responses the service domain can produce.
#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = ServiceResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum ServiceResponse {
    ServiceInstalled(ServiceName),
    ServiceUninstalled,
    ServiceStarted,
    ServiceHealthy(ServiceAddress),
    ServiceStopped,
    ServiceRunning(ServiceAddress),
    ServiceNotRunning(ServiceReason),
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
