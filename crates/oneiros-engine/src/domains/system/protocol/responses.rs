use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize)]
#[kinded(kind = SystemResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SystemResponse {
    SystemInitialized(TenantName),
    HostAlreadyInitialized,
}
