use crate::*;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TokenClaims {
    pub brain_id: BrainId,
    pub tenant_id: TenantId,
    pub actor_id: ActorId,
}
