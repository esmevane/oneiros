use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub(crate) struct TokenClaims {
    pub(crate) brain_id: BrainId,
    pub(crate) tenant_id: TenantId,
    pub(crate) actor_id: ActorId,
}
