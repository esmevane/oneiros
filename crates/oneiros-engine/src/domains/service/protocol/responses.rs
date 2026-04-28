use std::fmt;

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// All responses the service domain can produce.
#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = ServiceResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ServiceResponse {
    ServiceInstalled(ServiceInstalledResponse),
    ServiceUninstalled,
    ServiceStarted,
    ServiceHealthy(ServiceHealthyResponse),
    ServiceStopped,
    ServiceRunning(ServiceRunningResponse),
    ServiceNotRunning(ServiceNotRunningResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ServiceInstalledResponse {
        V1 => {
            #[builder(into)] pub name: String,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ServiceHealthyResponse {
        V1 => {
            #[builder(into)] pub address: String,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ServiceRunningResponse {
        V1 => {
            #[builder(into)] pub address: String,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum ServiceNotRunningResponse {
        V1 => {
            #[builder(into)] pub reason: String,
        }
    }
}

impl fmt::Display for ServiceResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceInstalled(ServiceInstalledResponse::V1(d)) => {
                write!(f, "Service installed as '{}'.", d.name)
            }
            Self::ServiceUninstalled => write!(f, "Service uninstalled."),
            Self::ServiceStarted => write!(f, "Service started."),
            Self::ServiceHealthy(ServiceHealthyResponse::V1(d)) => {
                write!(f, "Service started and healthy at {}.", d.address)
            }
            Self::ServiceStopped => write!(f, "Service stopped."),
            Self::ServiceRunning(ServiceRunningResponse::V1(d)) => {
                write!(f, "Service is running at {}.", d.address)
            }
            Self::ServiceNotRunning(ServiceNotRunningResponse::V1(d)) => {
                write!(f, "Service is not running: {}", d.reason)
            }
        }
    }
}
