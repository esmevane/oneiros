use std::fmt;

use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = HostResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum HostResponse {
    HostInitialized(HostInitializedResponse),
    HostAlreadyInitialized,
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
    pub(crate) enum HostInitializedResponse {
        V1 => {
            #[builder(into)] pub(crate) tenant: TenantName,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ServiceInstalledResponse {
        V1 => {
            #[builder(into)] pub(crate) name: String,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ServiceHealthyResponse {
        V1 => {
            #[builder(into)] pub(crate) address: String,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ServiceRunningResponse {
        V1 => {
            #[builder(into)] pub(crate) address: String,
        }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum ServiceNotRunningResponse {
        V1 => {
            #[builder(into)] pub(crate) reason: String,
        }
    }
}

impl fmt::Display for HostResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HostInitialized(HostInitializedResponse::V1(d)) => {
                write!(f, "Host initialized with tenant '{}'.", d.tenant)
            }
            Self::HostAlreadyInitialized => write!(f, "Host already initialized."),
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
