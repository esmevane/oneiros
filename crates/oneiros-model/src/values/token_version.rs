#[derive(serde::Serialize, serde::Deserialize)]
pub(super) enum TokenVersion {
    V0(super::claim::TokenClaims),
}

/// Legacy token format: raw UUIDs without the tagged `Id` enum wrapper.
/// Used to decode tokens created before the `Id` type became an enum.
#[derive(serde::Deserialize)]
pub(super) enum LegacyTokenVersion {
    V0(LegacyClaims),
}

#[derive(serde::Deserialize)]
pub(super) struct LegacyClaims {
    pub brain_id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub actor_id: uuid::Uuid,
}
