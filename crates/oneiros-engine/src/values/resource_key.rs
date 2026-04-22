use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::*;

/// A fetch-by-identifier selector: either the native key (Id or Name) or an
/// opaque cross-domain [`RefToken`].
///
/// Parses from strings as `ref:<...>` → [`ResourceKey::Ref`], anything else →
/// [`ResourceKey::Key`] via `K::from_str`. Resolves to `K` by destructuring
/// a `Ref` and enforcing it points at the expected resource kind.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResourceKey<K> {
    Key(K),
    Ref(RefToken),
}

impl<K> ResourceKey<K> {
    pub fn from_key(key: K) -> Self {
        Self::Key(key)
    }

    pub fn from_ref(r: Ref) -> Self {
        Self::Ref(RefToken::new(r))
    }
}

impl<K> From<K> for ResourceKey<K> {
    fn from(key: K) -> Self {
        Self::Key(key)
    }
}

impl<K> ResourceKey<K>
where
    K: Clone + TryFrom<Ref, Error = ResolveError>,
{
    /// Collapse either variant into the native key, returning a
    /// [`ResolveError::WrongKind`] when a ref points at the wrong resource.
    pub fn resolve(&self) -> Result<K, ResolveError> {
        match self {
            Self::Key(k) => Ok(k.clone()),
            Self::Ref(token) => K::try_from(token.inner().clone()),
        }
    }
}

/// Error raised when a [`ResourceKey::Ref`] cannot be turned into the native
/// key the caller expected.
#[derive(Debug, thiserror::Error)]
pub enum ResolveError {
    #[error("expected a {expected} ref, got a {got} ref")]
    WrongKind {
        expected: &'static str,
        got: &'static str,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ResourceKeyParseError<E: fmt::Debug + fmt::Display> {
    #[error("{0}")]
    Key(E),
    #[error(transparent)]
    Ref(#[from] RefError),
}

impl<K> FromStr for ResourceKey<K>
where
    K: FromStr,
    K::Err: fmt::Debug + fmt::Display,
{
    type Err = ResourceKeyParseError<K::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("ref:") {
            Ok(Self::Ref(s.parse::<RefToken>()?))
        } else {
            s.parse::<K>()
                .map(Self::Key)
                .map_err(ResourceKeyParseError::Key)
        }
    }
}

impl<K: fmt::Display> fmt::Display for ResourceKey<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(k) => write!(f, "{k}"),
            Self::Ref(token) => write!(f, "{token}"),
        }
    }
}

impl<K: fmt::Display> Serialize for ResourceKey<K> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, K> Deserialize<'de> for ResourceKey<K>
where
    K: FromStr,
    K::Err: fmt::Debug + fmt::Display,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl<K> schemars::JsonSchema for ResourceKey<K> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ResourceKey")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "description": "Either a native resource key (id or name) or an opaque ref token (ref:<base64url>)"
        })
    }
}

macro_rules! resource_key_try_from {
    ($ty:ty, $variant:ident, $label:literal) => {
        impl TryFrom<Ref> for $ty {
            type Error = ResolveError;

            fn try_from(r: Ref) -> Result<Self, Self::Error> {
                let Ref::V0(resource) = r;
                match resource {
                    Resource::$variant(inner) => Ok(inner),
                    other => Err(ResolveError::WrongKind {
                        expected: $label,
                        got: other.label(),
                    }),
                }
            }
        }
    };
}

resource_key_try_from!(AgentId, Agent, "agent");
resource_key_try_from!(ActorId, Actor, "actor");
resource_key_try_from!(BookmarkId, Bookmark, "bookmark");
resource_key_try_from!(BrainId, Brain, "brain");
resource_key_try_from!(CognitionId, Cognition, "cognition");
resource_key_try_from!(ConnectionId, Connection, "connection");
resource_key_try_from!(ExperienceId, Experience, "experience");
resource_key_try_from!(FollowId, Follow, "follow");
resource_key_try_from!(LevelName, Level, "level");
resource_key_try_from!(MemoryId, Memory, "memory");
resource_key_try_from!(NatureName, Nature, "nature");
resource_key_try_from!(PeerId, Peer, "peer");
resource_key_try_from!(PersonaName, Persona, "persona");
resource_key_try_from!(SensationName, Sensation, "sensation");
resource_key_try_from!(StorageKey, Storage, "storage");
resource_key_try_from!(TenantId, Tenant, "tenant");
resource_key_try_from!(TextureName, Texture, "texture");
resource_key_try_from!(TicketId, Ticket, "ticket");
resource_key_try_from!(UrgeName, Urge, "urge");

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_id_string_as_key_variant() {
        let id = ExperienceId::new();
        let parsed: ResourceKey<ExperienceId> = id.to_string().parse().unwrap();
        assert_eq!(parsed, ResourceKey::Key(id));
    }

    #[test]
    fn parses_ref_string_as_ref_variant() {
        let id = ExperienceId::new();
        let token = RefToken::new(Ref::experience(id));
        let parsed: ResourceKey<ExperienceId> = token.to_string().parse().unwrap();
        assert_eq!(parsed, ResourceKey::Ref(token));
    }

    #[test]
    fn parses_name_string_as_key_variant() {
        let parsed: ResourceKey<AgentName> = "scribe.process".parse().unwrap();
        assert_eq!(parsed, ResourceKey::Key(AgentName::new("scribe.process")));
    }

    #[test]
    fn parses_ref_string_for_name_keyed_resource() {
        let name = TextureName::new("observation");
        let token = RefToken::new(Ref::texture(name.clone()));
        let parsed: ResourceKey<TextureName> = token.to_string().parse().unwrap();
        assert_eq!(parsed, ResourceKey::Ref(token));
    }

    #[test]
    fn resolve_returns_key_variant_directly() {
        let id = ExperienceId::new();
        let key = ResourceKey::Key(id);
        assert_eq!(key.resolve().unwrap(), id);
    }

    #[test]
    fn resolve_unwraps_matching_ref() {
        let id = ExperienceId::new();
        let key: ResourceKey<ExperienceId> = ResourceKey::from_ref(Ref::experience(id));
        assert_eq!(key.resolve().unwrap(), id);
    }

    #[test]
    fn resolve_errors_on_wrong_kind_ref() {
        let cognition_ref = Ref::cognition(CognitionId::new());
        let key: ResourceKey<ExperienceId> = ResourceKey::from_ref(cognition_ref);
        let err = key.resolve().unwrap_err();
        assert!(matches!(
            err,
            ResolveError::WrongKind {
                expected: "experience",
                got: "cognition"
            }
        ));
    }

    #[test]
    fn resolve_errors_on_name_keyed_wrong_kind() {
        let urge_ref = Ref::urge(UrgeName::new("catharsis"));
        let key: ResourceKey<TextureName> = ResourceKey::from_ref(urge_ref);
        let err = key.resolve().unwrap_err();
        assert!(matches!(
            err,
            ResolveError::WrongKind {
                expected: "texture",
                got: "urge"
            }
        ));
    }

    #[test]
    fn malformed_id_fails_as_key_parse_error() {
        let result: Result<ResourceKey<ExperienceId>, _> = "not-a-uuid".parse();
        assert!(matches!(result, Err(ResourceKeyParseError::Key(_))));
    }

    #[test]
    fn malformed_ref_fails_as_ref_parse_error() {
        let result: Result<ResourceKey<ExperienceId>, _> = "ref:!!!bad!!!".parse();
        assert!(matches!(result, Err(ResourceKeyParseError::Ref(_))));
    }

    #[test]
    fn display_roundtrips_key_variant() {
        let id = ExperienceId::new();
        let key = ResourceKey::Key(id);
        let parsed: ResourceKey<ExperienceId> = key.to_string().parse().unwrap();
        assert_eq!(parsed, key);
    }

    #[test]
    fn display_roundtrips_ref_variant() {
        let id = ExperienceId::new();
        let key: ResourceKey<ExperienceId> = ResourceKey::from_ref(Ref::experience(id));
        let parsed: ResourceKey<ExperienceId> = key.to_string().parse().unwrap();
        assert_eq!(parsed, key);
    }

    #[test]
    fn serde_roundtrip_key_variant() {
        let id = ExperienceId::new();
        let key = ResourceKey::Key(id);
        let json = serde_json::to_string(&key).unwrap();
        assert!(json.starts_with('"'));
        let decoded: ResourceKey<ExperienceId> = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, key);
    }

    #[test]
    fn serde_roundtrip_ref_variant() {
        let id = ExperienceId::new();
        let key: ResourceKey<ExperienceId> = ResourceKey::from_ref(Ref::experience(id));
        let json = serde_json::to_string(&key).unwrap();
        let decoded: ResourceKey<ExperienceId> = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, key);
    }

    #[test]
    fn try_from_ref_roundtrip_all_id_keyed() {
        macro_rules! check_id {
            ($id:expr, $variant:ident) => {{
                let id = $id;
                let r = Ref::V0(Resource::$variant(id));
                let back = <_ as TryFrom<Ref>>::try_from(r).unwrap();
                assert_eq!(id, back);
            }};
        }
        check_id!(AgentId::new(), Agent);
        check_id!(ActorId::new(), Actor);
        check_id!(BookmarkId::new(), Bookmark);
        check_id!(BrainId::new(), Brain);
        check_id!(CognitionId::new(), Cognition);
        check_id!(ConnectionId::new(), Connection);
        check_id!(ExperienceId::new(), Experience);
        check_id!(FollowId::new(), Follow);
        check_id!(MemoryId::new(), Memory);
        check_id!(PeerId::new(), Peer);
        check_id!(TenantId::new(), Tenant);
        check_id!(TicketId::new(), Ticket);
    }

    #[test]
    fn try_from_ref_roundtrip_all_name_keyed() {
        macro_rules! check_name {
            ($name:expr, $variant:ident) => {{
                let name = $name;
                let r = Ref::V0(Resource::$variant(name.clone()));
                let back = <_ as TryFrom<Ref>>::try_from(r).unwrap();
                assert_eq!(name, back);
            }};
        }
        check_name!(LevelName::new("core"), Level);
        check_name!(NatureName::new("context"), Nature);
        check_name!(PersonaName::new("process"), Persona);
        check_name!(SensationName::new("echoes"), Sensation);
        check_name!(StorageKey::new("config.toml"), Storage);
        check_name!(TextureName::new("observation"), Texture);
        check_name!(UrgeName::new("catharsis"), Urge);
    }
}
