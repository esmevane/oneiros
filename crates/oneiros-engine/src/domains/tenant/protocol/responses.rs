use kinded::Kinded;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Kinded, Serialize, Deserialize, JsonSchema)]
#[kinded(kind = TenantResponseType, display = "kebab-case")]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TenantResponse {
    Created(TenantCreatedResponse),
    Found(TenantFoundResponse),
    Listed(TenantsResponse),
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TenantCreatedResponse {
        V1 => { #[serde(flatten)] pub tenant: Tenant }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TenantFoundResponse {
        V1 => { #[serde(flatten)] pub tenant: Tenant }
    }
}

versioned! {
    #[derive(JsonSchema)]
    pub enum TenantsResponse {
        V1 => {
            pub items: Vec<Tenant>,
            pub total: usize,
        }
    }
}
