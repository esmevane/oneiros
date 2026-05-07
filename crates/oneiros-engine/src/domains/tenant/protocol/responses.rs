use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = TenantResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub(crate) enum TenantResponse {
    Created(TenantCreatedResponse),
    Found(TenantFoundResponse),
    Listed(TenantsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TenantCreatedResponse {
        V1 => { #[serde(flatten)] pub(crate) tenant: Tenant }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TenantFoundResponse {
        V1 => { #[serde(flatten)] pub(crate) tenant: Tenant }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub(crate) enum TenantsResponse {
        V1 => {
            pub(crate) items: Vec<Tenant>,
            pub(crate) total: usize,
        }
    }
}
