#[derive(serde::Serialize, serde::Deserialize)]
pub(super) enum TokenVersion {
    V0(super::claim::TokenClaims),
}

/// Enum-era token format: tokens issued while `Id` was a tagged enum
/// (`Legacy(Uuid)` / `Content(Bytes)`). These tokens carry an extra
/// variant discriminant per ID in their postcard bytes.
#[derive(serde::Deserialize)]
pub(super) enum EnumEraTokenVersion {
    V0(EnumEraClaims),
}

#[derive(serde::Deserialize)]
pub(super) struct EnumEraClaims {
    pub brain_id: EnumEraId,
    pub tenant_id: EnumEraId,
    pub actor_id: EnumEraId,
}

#[derive(serde::Deserialize)]
pub(super) enum EnumEraId {
    Legacy(uuid::Uuid),
    /// Exists for postcard deserialization of tokens issued during the Id-as-enum era.
    /// Tokens with Content IDs will fail to decode (returns TokenError::Encoding).
    Content(()),
}

impl EnumEraId {
    pub fn into_uuid(self) -> Option<uuid::Uuid> {
        match self {
            Self::Legacy(uuid) => Some(uuid),
            Self::Content(_) => None,
        }
    }
}
