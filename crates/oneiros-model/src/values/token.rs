use super::claim::TokenClaims as Claim;
use super::token_version::{LegacyTokenVersion, TokenVersion};
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
    /// Tries the current tagged format first. If that fails, falls back to
    /// the legacy format (raw UUID bytes without `Id` enum variant tags).
    pub fn decode(&self) -> Result<Claim, TokenError> {
        let upper = self.0.to_uppercase();
        let bytes = data_encoding::BASE32_NOPAD
            .decode(upper.as_bytes())
            .map_err(|_| TokenError::Encoding)?;

        if let Ok(versioned) = postcard::from_bytes::<TokenVersion>(&bytes) {
            let TokenVersion::V0(claims) = versioned;
            return Ok(claims);
        }

        let legacy: LegacyTokenVersion = postcard::from_bytes(&bytes)?;
        let LegacyTokenVersion::V0(legacy_claims) = legacy;

        Ok(Claim {
            brain_id: BrainId(Id::Legacy(legacy_claims.brain_id)),
            tenant_id: TenantId(Id::Legacy(legacy_claims.tenant_id)),
            actor_id: ActorId(Id::Legacy(legacy_claims.actor_id)),
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
    fn legacy_token_decode() {
        // Reproduce the old token format: the old Id was `struct Id(Uuid)`
        // with `#[serde(transparent)]`, so postcard serialized each ID as
        // a raw UUID (which uses serialize_bytes = length-prefixed).
        // We use a mirror struct to produce those exact bytes.
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
        let encoded = data_encoding::BASE32_NOPAD
            .encode(&payload)
            .to_lowercase();
        let token = Token(encoded);

        let decoded = token.decode().expect("legacy token should decode via fallback");

        assert_eq!(decoded.brain_id, BrainId(Id::Legacy(brain_uuid)));
        assert_eq!(decoded.tenant_id, TenantId(Id::Legacy(tenant_uuid)));
        assert_eq!(decoded.actor_id, ActorId(Id::Legacy(actor_uuid)));
    }
}
