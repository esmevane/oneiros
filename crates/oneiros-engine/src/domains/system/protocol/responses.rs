use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = SystemResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum SystemResponse {
    SystemInitialized(SystemInitializedResponse),
    HostAlreadyInitialized,
}

versioned! {
    #[derive(JsonSchema)]
    pub enum SystemInitializedResponse {
        V1 => {
            #[builder(into)] pub tenant: TenantName,
        }
    }
}
