use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Tenant {
    pub id: TenantId,
    pub name: TenantName,
    pub created_at: String,
}

resource_id!(TenantId);
resource_name!(TenantName);
