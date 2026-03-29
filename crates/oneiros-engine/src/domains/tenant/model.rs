use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Tenant {
    #[builder(default, into)]
    pub id: TenantId,
    #[builder(into)]
    pub name: TenantName,
    #[builder(default = Timestamp::now(), into)]
    pub created_at: Timestamp,
}

resource_id!(TenantId);
resource_name!(TenantName);
