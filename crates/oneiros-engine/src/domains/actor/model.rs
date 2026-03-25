use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Actor {
    pub id: ActorId,
    pub tenant_id: TenantId,
    pub name: ActorName,
    pub created_at: Timestamp,
}

resource_id!(ActorId);
resource_name!(ActorName);
