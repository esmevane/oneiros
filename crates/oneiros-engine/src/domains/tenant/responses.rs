use serde::{Deserialize, Serialize};

use super::model::Tenant;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TenantResponse {
    Created(Tenant),
    Found(Tenant),
    Listed(Vec<Tenant>),
}
