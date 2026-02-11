use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize)]
pub struct Actor {
    pub tenant_id: TenantId,
    pub actor_id: ActorId,
    pub name: ActorName,
}

domain_id!(ActorId);
domain_name!(ActorName);
