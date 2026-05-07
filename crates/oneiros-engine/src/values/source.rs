use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub(crate) struct Source {
    pub(crate) actor_id: ActorId,
    pub(crate) tenant_id: TenantId,
}
