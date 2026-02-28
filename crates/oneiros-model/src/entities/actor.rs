use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Actor {
    pub id: ActorId,
    pub tenant_id: TenantId,
    pub name: ActorName,
}

impl Actor {
    pub fn init(tenant_id: TenantId, name: ActorName) -> Self {
        Self {
            id: ActorId::new(),
            tenant_id,
            name,
        }
    }
}

domain_id!(ActorId);
domain_name!(ActorName);
