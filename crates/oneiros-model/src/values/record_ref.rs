use oneiros_link::{Key, Link};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::*;

/// A reference to another entity, carrying either an ID, a content-addressed
/// Link, or both, plus an optional role label.
///
/// Replaces the former `RecordRef` untagged enum. Backward-compatible with
/// legacy JSON shapes:
///   - `{ "id": "...", "kind": "cognition", "role": "origin" }` (kind is accepted but dropped)
///   - `{ "link": "base64url...", "role": "origin" }`
///   - `{ "id": "...", "link": "...", "role": "..." }` (both present)
///   - `{ "id": "...", "role": "..." }` (no kind, no link)
#[derive(Clone, Debug, PartialEq)]
pub struct EntityRef {
    key: Key<Id, Link>,
    role: Option<Label>,
}

impl EntityRef {
    pub fn from_id(id: Id, role: Option<Label>) -> Self {
        Self {
            key: Key::Id(id),
            role,
        }
    }

    pub fn from_link(link: Link, role: Option<Label>) -> Self {
        Self {
            key: Key::Link(link),
            role,
        }
    }

    pub fn from_both(id: Id, link: Link, role: Option<Label>) -> Self {
        Self {
            key: Key::Both(id, link),
            role,
        }
    }

    pub fn key(&self) -> &Key<Id, Link> {
        &self.key
    }

    pub fn id(&self) -> Option<&Id> {
        self.key.try_id()
    }

    pub fn link(&self) -> Option<&Link> {
        self.key.try_link()
    }

    pub fn role(&self) -> Option<&Label> {
        self.role.as_ref()
    }
}

impl core::fmt::Display for EntityRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Prefer link display (more informative); fall back to short ID.
        let identity = match &self.key {
            Key::Link(link) | Key::Both(_, link) => format!("{link}"),
            Key::Id(id) => {
                let s = id.to_string();
                s[..s.len().min(8)].to_string()
            }
        };

        match &self.role {
            Some(role) => write!(f, "{identity} [{role}]"),
            None => write!(f, "{identity}"),
        }
    }
}

// -- Serde --

/// Helper for deserializing EntityRef from multiple JSON shapes.
#[derive(Deserialize)]
struct EntityRefDeHelper {
    #[serde(default)]
    id: Option<Id>,
    #[serde(default)]
    link: Option<Link>,
    /// Accepted for backward compat with legacy `{id, kind, role}` shape.
    /// Not stored in EntityRef — the Link carries the resource type.
    #[serde(default)]
    #[allow(dead_code)]
    kind: Option<RecordKind>,
    #[serde(default)]
    role: Option<Label>,
}

impl<'de> Deserialize<'de> for EntityRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let helper = EntityRefDeHelper::deserialize(deserializer)?;

        let key = match (helper.id, helper.link) {
            (Some(id), Some(link)) => Key::Both(id, link),
            (None, Some(link)) => Key::Link(link),
            (Some(id), None) => Key::Id(id),
            (None, None) => {
                return Err(serde::de::Error::custom(
                    "EntityRef requires at least one of 'id' or 'link'",
                ))
            }
        };

        Ok(EntityRef {
            key,
            role: helper.role,
        })
    }
}

/// Helper for serializing EntityRef as a flat JSON object.
#[derive(Serialize)]
struct EntityRefSerHelper<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<&'a Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<&'a Label>,
}

impl Serialize for EntityRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        EntityRefSerHelper {
            id: self.key.try_id(),
            link: self.key.try_link(),
            role: self.role.as_ref(),
        }
        .serialize(serializer)
    }
}

// -- Construction from DB rows --

#[derive(Debug, Error)]
pub enum EntityRefConstructionError {
    #[error("invalid record id: {0}")]
    InvalidId(#[from] IdParseError),
    #[error("invalid link: {0}")]
    InvalidLink(#[from] oneiros_link::LinkError),
}

impl EntityRef {
    /// Construct from DB row: (record_id, record_kind, role, link).
    ///
    /// Prefers link when present. Falls back to record_id.
    /// record_kind is accepted but not stored (backward compat).
    pub fn construct_from_db(
        record_id: Option<String>,
        _record_kind: Option<String>,
        role: Option<String>,
        link: Option<String>,
    ) -> Result<Self, EntityRefConstructionError> {
        let role = role.map(Label::new);

        let parsed_link = link
            .as_deref()
            .map(|s| s.parse::<Link>())
            .transpose()?;

        let parsed_id = record_id
            .as_deref()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<Id>())
            .transpose()?;

        let key = match (parsed_id, parsed_link) {
            (Some(id), Some(link)) => Key::Both(id, link),
            (None, Some(link)) => Key::Link(link),
            (Some(id), None) => Key::Id(id),
            (None, None) => {
                // Shouldn't happen with valid DB data, but handle gracefully.
                return Err(EntityRefConstructionError::InvalidId(
                    "empty".parse::<Id>().unwrap_err(),
                ));
            }
        };

        Ok(EntityRef { key, role })
    }
}

// -- RecordKind (kept for CLI prefix-ID resolution) --

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecordKind {
    Cognition,
    Memory,
    Experience,
    Storage,
}

impl core::fmt::Display for RecordKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RecordKind::Cognition => write!(f, "cognition"),
            RecordKind::Memory => write!(f, "memory"),
            RecordKind::Experience => write!(f, "experience"),
            RecordKind::Storage => write!(f, "storage"),
        }
    }
}

#[derive(Debug, Error)]
#[error("unknown record kind: {0}")]
pub struct RecordKindParseError(pub String);

impl core::str::FromStr for RecordKind {
    type Err = RecordKindParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cognition" => Ok(RecordKind::Cognition),
            "memory" => Ok(RecordKind::Memory),
            "experience" => Ok(RecordKind::Experience),
            "storage" => Ok(RecordKind::Storage),
            other => Err(RecordKindParseError(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_id_roundtrips() {
        let r = EntityRef::from_id(Id::new(), Some(Label::new("origin")));
        let json = serde_json::to_string(&r).unwrap();
        let deserialized: EntityRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r, deserialized);
    }

    #[test]
    fn from_link_roundtrips() {
        let link = Link::new(&("cognition", "some-content")).unwrap();
        let r = EntityRef::from_link(link, Some(Label::new("origin")));
        let json = serde_json::to_string(&r).unwrap();
        let deserialized: EntityRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r, deserialized);
    }

    #[test]
    fn from_both_roundtrips() {
        let link = Link::new(&("cognition", "some-content")).unwrap();
        let r = EntityRef::from_both(Id::new(), link, Some(Label::new("origin")));
        let json = serde_json::to_string(&r).unwrap();
        let deserialized: EntityRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r, deserialized);
    }

    #[test]
    fn deserializes_from_legacy_identified_json() {
        let id = Id::new();
        let json = format!(r#"{{"id":"{}","kind":"cognition","role":"origin"}}"#, id);
        let r: EntityRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r.id(), Some(&id));
        assert_eq!(r.role().map(|l| l.as_str()), Some("origin"));
        // kind is accepted but not stored
        assert!(r.link().is_none());
    }

    #[test]
    fn deserializes_from_link_json() {
        let link = Link::new(&("memory", "some-content")).unwrap();
        let json = format!(r#"{{"link":"{}","role":"origin"}}"#, link);
        let r: EntityRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r.link(), Some(&link));
        assert_eq!(r.role().map(|l| l.as_str()), Some("origin"));
        assert!(r.id().is_none());
    }

    #[test]
    fn mixed_refs_in_vec() {
        let id = Id::new();
        let link = Link::new(&("cognition", "a-thought")).unwrap();
        let refs = vec![
            EntityRef::from_id(id.clone(), Some(Label::new("origin"))),
            EntityRef::from_link(link.clone(), Some(Label::new("echo"))),
        ];
        let json = serde_json::to_string(&refs).unwrap();
        let deserialized: Vec<EntityRef> = serde_json::from_str(&json).unwrap();
        assert_eq!(refs, deserialized);
    }

    #[test]
    fn without_role() {
        let link = Link::new(&("experience", "thread")).unwrap();
        let r = EntityRef::from_link(link.clone(), None);
        let json = serde_json::to_string(&r).unwrap();
        let deserialized: EntityRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r, deserialized);
        assert!(deserialized.role().is_none());
    }

    #[test]
    fn display_id_only() {
        let r = EntityRef::from_id(Id::new(), Some(Label::new("origin")));
        let display = r.to_string();
        assert!(display.contains("[origin]"));
    }

    #[test]
    fn display_link() {
        let link = Link::new(&("cognition", "thought")).unwrap();
        let r = EntityRef::from_link(link.clone(), Some(Label::new("echo")));
        let display = r.to_string();
        assert!(display.contains(&link.to_string()));
        assert!(display.contains("[echo]"));
    }

    #[test]
    fn rejects_empty_ref() {
        let json = r#"{"role":"origin"}"#;
        let result: Result<EntityRef, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
