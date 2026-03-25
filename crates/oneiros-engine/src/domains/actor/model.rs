use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Actor {
    #[builder(default)]
    pub id: ActorId,
    pub tenant_id: TenantId,
    #[builder(into)]
    pub name: ActorName,
    #[builder(default = Timestamp::now(), into)]
    pub created_at: Timestamp,
}

resource_id!(ActorId);
resource_name!(ActorName);
