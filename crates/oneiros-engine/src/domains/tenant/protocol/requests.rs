use clap::Args;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct CreateTenant {
    pub name: TenantName,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Args)]
pub struct GetTenant {
    pub id: TenantId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum TenantRequest {
    Create(CreateTenant),
    Get(GetTenant),
    List,
}
