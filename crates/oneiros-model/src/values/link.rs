use serde::{Deserialize, Serialize};

use crate::*;

/// A scoped address pointing to a record, with an optional relationship role.
///
/// Links replace the flat RecordRef type with a two-variant enum that
/// distinguishes local (same brain) from remote (cross-brain) references.
/// This lays the groundwork for inter-brain communication and mailboxes.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Link {
    Local {
        resource: ResourceRef,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        role: Option<Label>,
    },
    Remote {
        brain: BrainRef,
        resource: ResourceRef,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        role: Option<Label>,
    },
}

impl Link {
    pub fn local(resource: ResourceRef) -> Self {
        Link::Local {
            resource,
            role: None,
        }
    }

    pub fn local_with_role(resource: ResourceRef, role: Label) -> Self {
        Link::Local {
            resource,
            role: Some(role),
        }
    }

    pub fn resource(&self) -> &ResourceRef {
        match self {
            Link::Local { resource, .. } => resource,
            Link::Remote { resource, .. } => resource,
        }
    }

    pub fn role(&self) -> Option<&Label> {
        match self {
            Link::Local { role, .. } => role.as_ref(),
            Link::Remote { role, .. } => role.as_ref(),
        }
    }

    pub fn brain(&self) -> Option<&BrainRef> {
        match self {
            Link::Local { .. } => None,
            Link::Remote { brain, .. } => Some(brain),
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Link::Local { .. })
    }

    pub fn is_remote(&self) -> bool {
        matches!(self, Link::Remote { .. })
    }
}

/// A typed reference to a specific record within a brain.
///
/// Unlike the old `RecordKind + Id` pair, ResourceRef carries the ID
/// inside the variant, making it impossible to mismatch kind and ID type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResourceRef {
    Agent(AgentId),
    Cognition(CognitionId),
    Memory(MemoryId),
    Experience(ExperienceId),
    Storage(StorageKey),
}

impl ResourceRef {
    /// The kind name as a string, matching the old RecordKind display format.
    pub fn kind_name(&self) -> &'static str {
        match self {
            ResourceRef::Agent(_) => "agent",
            ResourceRef::Cognition(_) => "cognition",
            ResourceRef::Memory(_) => "memory",
            ResourceRef::Experience(_) => "experience",
            ResourceRef::Storage(_) => "storage",
        }
    }

    /// The ID as a string, suitable for DB storage.
    pub fn id_string(&self) -> String {
        match self {
            ResourceRef::Agent(id) => id.to_string(),
            ResourceRef::Cognition(id) => id.to_string(),
            ResourceRef::Memory(id) => id.to_string(),
            ResourceRef::Experience(id) => id.to_string(),
            ResourceRef::Storage(key) => key.as_str().to_string(),
        }
    }
}

impl core::fmt::Display for ResourceRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let id_str = self.id_string();
        let short = if id_str.len() >= 8 {
            &id_str[..8]
        } else {
            &id_str
        };
        write!(f, "{}:{}", short, self.kind_name())
    }
}

impl core::fmt::Display for Link {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Link::Local { resource, role } => match role {
                Some(role) => write!(f, "{resource} [{role}]"),
                None => write!(f, "{resource}"),
            },
            Link::Remote {
                brain,
                resource,
                role,
            } => match role {
                Some(role) => write!(f, "{brain}/{resource} [{role}]"),
                None => write!(f, "{brain}/{resource}"),
            },
        }
    }
}

/// A reference to another brain, for cross-brain links.
///
/// For now this is a simple string wrapper. When deterministic IDs land,
/// this will evolve to carry content-addressed brain identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BrainRef(pub String);

impl BrainRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for BrainRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

// -- Backward-compatible deserialization --

/// Legacy RecordRef shape from old events in the log.
#[derive(Deserialize)]
struct LegacyRecordRef {
    id: Id,
    kind: RecordKind,
    #[serde(default)]
    role: Option<Label>,
}

impl From<LegacyRecordRef> for Link {
    fn from(legacy: LegacyRecordRef) -> Self {
        let resource = match legacy.kind {
            RecordKind::Cognition => ResourceRef::Cognition(CognitionId(legacy.id)),
            RecordKind::Memory => ResourceRef::Memory(MemoryId(legacy.id)),
            RecordKind::Experience => ResourceRef::Experience(ExperienceId(legacy.id)),
            RecordKind::Storage => ResourceRef::Storage(StorageKey::new(legacy.id.to_string())),
        };
        Link::Local {
            resource,
            role: legacy.role,
        }
    }
}

/// Helper enum for untagged deserialization — tries new format first,
/// falls back to legacy RecordRef.
#[derive(Deserialize)]
#[serde(untagged)]
enum LinkRepr {
    Modern(ModernLink),
    Legacy(LegacyRecordRef),
}

/// The "modern" Link shape for serialization/deserialization.
/// Mirrors Link but derives both Serialize and Deserialize directly.
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ModernLink {
    Local {
        resource: ResourceRef,
        #[serde(default)]
        role: Option<Label>,
    },
    Remote {
        brain: BrainRef,
        resource: ResourceRef,
        #[serde(default)]
        role: Option<Label>,
    },
}

impl From<ModernLink> for Link {
    fn from(modern: ModernLink) -> Self {
        match modern {
            ModernLink::Local { resource, role } => Link::Local { resource, role },
            ModernLink::Remote {
                brain,
                resource,
                role,
            } => Link::Remote {
                brain,
                resource,
                role,
            },
        }
    }
}

impl<'de> Deserialize<'de> for Link {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let repr = LinkRepr::deserialize(deserializer)?;
        Ok(match repr {
            LinkRepr::Modern(modern) => modern.into(),
            LinkRepr::Legacy(legacy) => legacy.into(),
        })
    }
}

// -- Construction from DB rows --

#[derive(Debug, thiserror::Error)]
pub enum LinkConstructionError {
    #[error("invalid record id: {0}")]
    InvalidId(#[from] IdParseError),
    #[error("unknown resource kind: {0}")]
    UnknownKind(String),
}

impl Link {
    /// Construct a local Link from DB row values (record_id, record_kind, role).
    pub fn construct_from_db(
        record_id: &str,
        record_kind: &str,
        role: Option<String>,
    ) -> Result<Self, LinkConstructionError> {
        let id: Id = record_id.parse()?;
        let resource = match record_kind {
            "agent" => ResourceRef::Agent(AgentId(id)),
            "cognition" => ResourceRef::Cognition(CognitionId(id)),
            "memory" => ResourceRef::Memory(MemoryId(id)),
            "experience" => ResourceRef::Experience(ExperienceId(id)),
            "storage" => ResourceRef::Storage(StorageKey::new(record_id)),
            other => return Err(LinkConstructionError::UnknownKind(other.to_string())),
        };
        Ok(Link::Local {
            resource,
            role: role.map(Label::new),
        })
    }
}

// -- Encoding (postcard + BASE64URL, versioned) --

#[derive(Serialize, Deserialize)]
enum LinkVersion {
    V0(ModernLink),
}

#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    #[error("invalid link encoding")]
    Encoding,

    #[error("invalid link format: {0}")]
    Format(#[from] postcard::Error),
}

impl Link {
    /// Encode this link as a URL-safe BASE64URL string (versioned via postcard).
    pub fn encode(&self) -> String {
        let modern = match self.clone() {
            Link::Local { resource, role } => ModernLink::Local { resource, role },
            Link::Remote {
                brain,
                resource,
                role,
            } => ModernLink::Remote {
                brain,
                resource,
                role,
            },
        };
        let versioned = LinkVersion::V0(modern);
        let bytes = postcard::to_allocvec(&versioned).expect("link serialization should not fail");
        data_encoding::BASE64URL_NOPAD.encode(&bytes)
    }

    /// Decode a link from a BASE64URL-encoded string.
    pub fn decode(encoded: &str) -> Result<Self, LinkError> {
        let bytes = data_encoding::BASE64URL_NOPAD
            .decode(encoded.as_bytes())
            .map_err(|_| LinkError::Encoding)?;
        let versioned: LinkVersion = postcard::from_bytes(&bytes)?;
        let LinkVersion::V0(modern) = versioned;
        Ok(modern.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_local_link() {
        let link = Link::local(ResourceRef::Cognition(CognitionId::new()));
        let json = serde_json::to_string(&link).unwrap();
        assert!(json.contains("\"local\""));
        assert!(json.contains("\"cognition\""));
    }

    #[test]
    fn serialize_local_link_with_role() {
        let link =
            Link::local_with_role(ResourceRef::Memory(MemoryId::new()), Label::new("origin"));
        let json = serde_json::to_string(&link).unwrap();
        assert!(json.contains("\"local\""));
        assert!(json.contains("\"memory\""));
        assert!(json.contains("\"origin\""));
    }

    #[test]
    fn deserialize_modern_format() {
        let json = r#"{"local":{"resource":{"cognition":"019c5ea2-d06d-7bd1-9f6b-c41efaa08956"},"role":"origin"}}"#;
        let link: Link = serde_json::from_str(json).unwrap();
        assert!(link.is_local());
        assert_eq!(link.resource().kind_name(), "cognition");
        assert_eq!(link.role().map(Label::as_str), Some("origin"));
    }

    #[test]
    fn deserialize_legacy_record_ref_format() {
        let json =
            r#"{"id":"019c5ea2-d06d-7bd1-9f6b-c41efaa08956","kind":"cognition","role":"origin"}"#;
        let link: Link = serde_json::from_str(json).unwrap();
        assert!(link.is_local());
        assert_eq!(link.resource().kind_name(), "cognition");
        assert_eq!(link.role().map(Label::as_str), Some("origin"));
    }

    #[test]
    fn deserialize_legacy_without_role() {
        let json = r#"{"id":"019c5ea2-d06d-7bd1-9f6b-c41efaa08956","kind":"memory"}"#;
        let link: Link = serde_json::from_str(json).unwrap();
        assert!(link.is_local());
        assert_eq!(link.resource().kind_name(), "memory");
        assert!(link.role().is_none());
    }

    #[test]
    fn roundtrip_encode_decode() {
        let link = Link::local_with_role(
            ResourceRef::Cognition(CognitionId::new()),
            Label::new("test"),
        );
        let encoded = link.encode();
        let decoded = Link::decode(&encoded).unwrap();
        assert_eq!(link, decoded);
    }

    #[test]
    fn roundtrip_encode_decode_remote() {
        let link = Link::Remote {
            brain: BrainRef::new("other-brain"),
            resource: ResourceRef::Experience(ExperienceId::new()),
            role: Some(Label::new("context")),
        };
        let encoded = link.encode();
        let decoded = Link::decode(&encoded).unwrap();
        assert_eq!(link, decoded);
    }

    #[test]
    fn display_local_link() {
        let id = CognitionId::new();
        let link = Link::local_with_role(ResourceRef::Cognition(id), Label::new("origin"));
        let display = link.to_string();
        assert!(display.contains("cognition"));
        assert!(display.contains("[origin]"));
    }

    #[test]
    fn display_remote_link() {
        let id = MemoryId::new();
        let link = Link::Remote {
            brain: BrainRef::new("other-brain"),
            resource: ResourceRef::Memory(id),
            role: None,
        };
        let display = link.to_string();
        assert!(display.contains("other-brain"));
        assert!(display.contains("memory"));
    }

    #[test]
    fn construct_from_db_cognition() {
        let id = CognitionId::new();
        let link =
            Link::construct_from_db(&id.to_string(), "cognition", Some("origin".to_string()))
                .unwrap();
        assert!(link.is_local());
        assert_eq!(link.resource().kind_name(), "cognition");
        assert_eq!(link.role().map(Label::as_str), Some("origin"));
    }

    #[test]
    fn construct_from_db_unknown_kind() {
        let id = Id::new();
        let result = Link::construct_from_db(&id.to_string(), "unknown", None);
        assert!(result.is_err());
    }

    #[test]
    fn resource_ref_display() {
        let id = CognitionId::new();
        let resource = ResourceRef::Cognition(id);
        let display = resource.to_string();
        assert!(display.contains("cognition"));
        assert!(display.len() > 10); // short_id:kind
    }

    #[test]
    fn deserialize_vec_of_mixed_formats() {
        let json = r#"[
            {"local":{"resource":{"cognition":"019c5ea2-d06d-7bd1-9f6b-c41efaa08956"}}},
            {"id":"019c5ea5-fbba-7c10-8496-3629de0dd5a5","kind":"memory","role":"origin"}
        ]"#;
        let links: Vec<Link> = serde_json::from_str(json).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].resource().kind_name(), "cognition");
        assert_eq!(links[1].resource().kind_name(), "memory");
        assert_eq!(links[1].role().map(Label::as_str), Some("origin"));
    }
}
