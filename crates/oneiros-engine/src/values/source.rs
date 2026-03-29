use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, schemars::JsonSchema)]
pub struct Source {
    pub actor_id: ActorId,
    pub tenant_id: TenantId,
}

impl Source {
    /// Construct a Source from decoded token claims.
    pub fn from_claims(claims: &TokenClaims) -> Self {
        Self {
            actor_id: claims.actor_id,
            tenant_id: claims.tenant_id,
        }
    }
}
