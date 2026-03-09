use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TenantEvents {
    TenantCreated(Tenant),
}

// ── Request types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTenantRequest {
    pub name: TenantName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTenantsRequest;

// ── Protocol enums ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TenantRequests {
    GetTenant(GetTenantRequest),
    ListTenants(ListTenantsRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum TenantResponses {
    TenantFound(Tenant),
    TenantsListed(Vec<Tenant>),
}
