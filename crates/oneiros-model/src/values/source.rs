use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Source {
    pub actor_id: ActorId,
    pub tenant_id: TenantId,
}
