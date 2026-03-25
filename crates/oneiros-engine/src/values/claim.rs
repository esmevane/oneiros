use bon::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Builder, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TokenClaims {
    pub brain_id: BrainId,
    pub tenant_id: TenantId,
    pub actor_id: ActorId,
}
