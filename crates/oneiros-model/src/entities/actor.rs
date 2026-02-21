use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Actor {
    pub tenant_id: TenantId,
    pub name: ActorName,
}

domain_link!(Actor => ActorLink);
domain_id!(ActorId);
domain_name!(ActorName);
