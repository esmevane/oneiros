use serde::Serialize;

use crate::*;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SystemResponse {
    SystemInitialized(TenantName),
    HostAlreadyInitialized,
}
