use super::claim::TokenClaims as Claim;
use super::token_version::{EnumEraTokenVersion, TokenVersion};
use crate::{ActorId, BrainId, Id, TenantId};

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Invalid token encoding")]
    Encoding,

    #[error("Invalid token format: {0}")]
    Format(#[from] postcard::Error),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Token(pub String);

impl Token {
    /// Issue a new token encoding the given claims.
    pub fn issue(claims: Claim) -> Self {
        let versioned = TokenVersion::V0(claims);
        let bytes = postcard::to_allocvec(&versioned).expect("token serialization should not fail");
        Self(data_encoding::BASE32_NOPAD.encode(&bytes).to_lowercase())
    }

    /// Decode this token's claims.
    ///
    /// Tries the current format first (transparent UUID wrapper). If that
    /// fails, falls back to the enum-era format (tokens issued while `Id`
    /// was a `Legacy(Uuid)` / `Content(Bytes)` tagged enum).
    pub fn decode(&self) -> Result<Claim, TokenError> {
        let upper = self.0.to_uppercase();
        let bytes = data_encoding::BASE32_NOPAD
            .decode(upper.as_bytes())
            .map_err(|_| TokenError::Encoding)?;

        // Try current format (Id as transparent UUID wrapper).
        if let Ok(versioned) = postcard::from_bytes::<TokenVersion>(&bytes) {
            let TokenVersion::V0(claims) = versioned;
            return Ok(claims);
        }

        // Fall back to enum-era format (Id was a tagged enum).
        let tagged: EnumEraTokenVersion = postcard::from_bytes(&bytes)?;
        let EnumEraTokenVersion::V0(claims) = tagged;

        Ok(Claim {
            brain_id: BrainId(Id(claims
                .brain_id
                .into_uuid()
                .ok_or(TokenError::Encoding)?)),
            tenant_id: TenantId(Id(claims
                .tenant_id
                .into_uuid()
                .ok_or(TokenError::Encoding)?)),
            actor_id: ActorId(Id(claims
                .actor_id
                .into_uuid()
                .ok_or(TokenError::Encoding)?)),
        })
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_round_trip() {
        let claims = Claim {
            brain_id: BrainId::new(),
            tenant_id: TenantId::new(),
            actor_id: ActorId::new(),
        };

        let token = Token::issue(claims.clone());
        let decoded = token.decode().unwrap();

        assert_eq!(claims, decoded);
    }

    #[test]
    fn enum_era_token_decode() {
        // Reproduce the enum-era token format: Id was `enum Id { Legacy(Uuid), Content(Bytes) }`
        // with a BinaryId helper. In postcard, each ID got a variant tag (0 for Legacy)
        // before the UUID bytes.
        #[derive(serde::Serialize)]
        enum EnumEraId {
            Legacy(uuid::Uuid),
            #[allow(dead_code)]
            Content(Vec<u8>),
        }

        #[derive(serde::Serialize)]
        enum OldTokenVersion {
            V0(OldClaims),
        }

        #[derive(serde::Serialize)]
        struct OldClaims {
            brain_id: EnumEraId,
            tenant_id: EnumEraId,
            actor_id: EnumEraId,
        }

        let brain_uuid = uuid::Uuid::now_v7();
        let tenant_uuid = uuid::Uuid::now_v7();
        let actor_uuid = uuid::Uuid::now_v7();

        let old_versioned = OldTokenVersion::V0(OldClaims {
            brain_id: EnumEraId::Legacy(brain_uuid),
            tenant_id: EnumEraId::Legacy(tenant_uuid),
            actor_id: EnumEraId::Legacy(actor_uuid),
        });

        let payload = postcard::to_allocvec(&old_versioned).unwrap();
        let encoded = data_encoding::BASE32_NOPAD.encode(&payload).to_lowercase();
        let token = Token(encoded);

        let decoded = token
            .decode()
            .expect("enum-era token should decode via fallback");

        assert_eq!(decoded.brain_id, BrainId(Id(brain_uuid)));
        assert_eq!(decoded.tenant_id, TenantId(Id(tenant_uuid)));
        assert_eq!(decoded.actor_id, ActorId(Id(actor_uuid)));
    }

    #[test]
    fn legacy_token_decode() {
        // Reproduce the original pre-enum token format: Id was `struct Id(Uuid)`
        // with `#[serde(transparent)]`, so postcard serialized each ID as
        // a raw UUID (which uses serialize_bytes = length-prefixed).
        #[derive(serde::Serialize)]
        enum OldTokenVersion {
            V0(OldClaims),
        }

        #[derive(serde::Serialize)]
        struct OldClaims {
            brain_id: uuid::Uuid,
            tenant_id: uuid::Uuid,
            actor_id: uuid::Uuid,
        }

        let brain_uuid = uuid::Uuid::now_v7();
        let tenant_uuid = uuid::Uuid::now_v7();
        let actor_uuid = uuid::Uuid::now_v7();

        let old_versioned = OldTokenVersion::V0(OldClaims {
            brain_id: brain_uuid,
            tenant_id: tenant_uuid,
            actor_id: actor_uuid,
        });

        let payload = postcard::to_allocvec(&old_versioned).unwrap();
        let encoded = data_encoding::BASE32_NOPAD.encode(&payload).to_lowercase();
        let token = Token(encoded);

        let decoded = token
            .decode()
            .expect("legacy token should decode via primary path");

        assert_eq!(decoded.brain_id, BrainId(Id(brain_uuid)));
        assert_eq!(decoded.tenant_id, TenantId(Id(tenant_uuid)));
        assert_eq!(decoded.actor_id, ActorId(Id(actor_uuid)));
    }
}
