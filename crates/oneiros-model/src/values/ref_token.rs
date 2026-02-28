use data_encoding::BASE64URL_NOPAD;
use serde::{Deserialize, Serialize};

use crate::*;

/// Opaque string encoding of a [`Ref`] for storage columns, CLI arguments,
/// and query parameters. Displayed as `ref:<base64url>`.
///
/// Use [`RefToken::new`] to wrap a `Ref`, and [`RefToken::into_inner`] to unwrap.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RefToken(Ref);

impl RefToken {
    pub fn new(r: Ref) -> Self {
        Self(r)
    }

    pub fn inner(&self) -> &Ref {
        &self.0
    }

    pub fn into_inner(self) -> Ref {
        self.0
    }
}

impl core::fmt::Display for RefToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ref:{}", BASE64URL_NOPAD.encode(&self.0.to_bytes()))
    }
}

impl core::str::FromStr for RefToken {
    type Err = RefError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let encoded = s.strip_prefix("ref:").unwrap_or(s);
        let bytes = BASE64URL_NOPAD.decode(encoded.as_bytes())?;
        Ok(Self(Ref::from_bytes(&bytes)?))
    }
}

impl Serialize for RefToken {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for RefToken {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl From<Ref> for RefToken {
    fn from(r: Ref) -> Self {
        Self(r)
    }
}

impl From<RefToken> for Ref {
    fn from(t: RefToken) -> Self {
        t.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ref_token_roundtrip_id_keyed() {
        let id = AgentId::new();
        let r = Ref::agent(id);
        let token = RefToken::new(r.clone());
        let encoded = token.to_string();
        assert!(encoded.starts_with("ref:"));
        let decoded: RefToken = encoded.parse().unwrap();
        assert_eq!(r, decoded.into_inner());
    }

    #[test]
    fn ref_token_roundtrip_name_keyed() {
        let r = Ref::texture(TextureName::new("observation"));
        let token = RefToken::new(r.clone());
        let encoded = token.to_string();
        let decoded: RefToken = encoded.parse().unwrap();
        assert_eq!(r, decoded.into_inner());
    }

    #[test]
    fn ref_token_accepts_without_prefix() {
        let r = Ref::level(LevelName::new("core"));
        let token = RefToken::new(r.clone());
        let full = token.to_string();
        // Strip "ref:" prefix and parse raw base64url
        let raw = full.strip_prefix("ref:").unwrap();
        let decoded: RefToken = raw.parse().unwrap();
        assert_eq!(r, decoded.into_inner());
    }

    #[test]
    fn distinctness_across_types() {
        let id = Id::new();
        let agent_ref = Ref::agent(AgentId::from(id));
        let cognition_ref = Ref::cognition(CognitionId::from(id));
        assert_ne!(agent_ref, cognition_ref);
        let agent_token = RefToken::new(agent_ref).to_string();
        let cognition_token = RefToken::new(cognition_ref).to_string();
        assert_ne!(agent_token, cognition_token);
    }

    #[test]
    fn deterministic_encoding() {
        let r = Ref::level(LevelName::new("project"));
        let t1 = RefToken::new(r.clone()).to_string();
        let t2 = RefToken::new(r).to_string();
        assert_eq!(t1, t2);
    }

    #[test]
    fn ref_token_serde_as_string() {
        let r = Ref::memory(MemoryId::new());
        let token = RefToken::new(r.clone());
        let json = serde_json::to_string(&token).unwrap();
        // Should be a quoted string
        assert!(json.starts_with('"'));
        assert!(json.ends_with('"'));

        let decoded: RefToken = serde_json::from_str(&json).unwrap();
        assert_eq!(r, decoded.into_inner());
    }

    #[test]
    fn invalid_base64_fails() {
        let result = "!!!invalid!!!".parse::<RefToken>();
        assert!(result.is_err());
    }

    #[test]
    fn invalid_postcard_fails() {
        let result = Ref::from_bytes(&[0xff, 0xff, 0xff]);
        assert!(result.is_err());
    }

    #[test]
    fn all_14_resource_types_roundtrip() {
        let refs = [
            Ref::agent(AgentId::new()),
            Ref::actor(ActorId::new()),
            Ref::brain(BrainId::new()),
            Ref::cognition(CognitionId::new()),
            Ref::connection(ConnectionId::new()),
            Ref::experience(ExperienceId::new()),
            Ref::level(LevelName::new("core")),
            Ref::memory(MemoryId::new()),
            Ref::nature(NatureName::new("context")),
            Ref::persona(PersonaName::new("process")),
            Ref::sensation(SensationName::new("echoes")),
            Ref::storage(StorageKey::new("config.toml")),
            Ref::tenant(TenantId::new()),
            Ref::texture(TextureName::new("observation")),
        ];

        for r in &refs {
            let token = RefToken::new(r.clone());
            let encoded = token.to_string();
            let decoded: RefToken = encoded.parse().unwrap();
            assert_eq!(r, decoded.inner(), "failed for {}", r.resource().label());
        }
    }
}
