use oneiros_link::Link;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentifiedRef {
    pub id: Id,
    pub kind: RecordKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Label>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedRef {
    pub link: Link,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Label>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordRef {
    Linked(LinkedRef),
    Identified(IdentifiedRef),
}

impl RecordRef {
    pub fn identified(id: Id, kind: RecordKind, role: Option<Label>) -> Self {
        RecordRef::Identified(IdentifiedRef { id, kind, role })
    }

    pub fn linked(link: Link, role: Option<Label>) -> Self {
        RecordRef::Linked(LinkedRef { link, role })
    }

    pub fn id(&self) -> Option<&Id> {
        match self {
            RecordRef::Identified(r) => Some(&r.id),
            RecordRef::Linked(_) => None,
        }
    }

    pub fn kind(&self) -> Option<&RecordKind> {
        match self {
            RecordRef::Identified(r) => Some(&r.kind),
            RecordRef::Linked(_) => None,
        }
    }

    pub fn link(&self) -> Option<&Link> {
        match self {
            RecordRef::Linked(r) => Some(&r.link),
            RecordRef::Identified(_) => None,
        }
    }

    pub fn role(&self) -> Option<&Label> {
        match self {
            RecordRef::Identified(r) => r.role.as_ref(),
            RecordRef::Linked(r) => r.role.as_ref(),
        }
    }
}

impl core::fmt::Display for RecordRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RecordRef::Identified(r) => {
                let short_id = &r.id.to_string()[..8];
                let kind = &r.kind;

                match &r.role {
                    Some(role) => write!(f, "{short_id}:{kind} [{role}]"),
                    None => write!(f, "{short_id}:{kind}"),
                }
            }
            RecordRef::Linked(r) => {
                let link = &r.link;

                match &r.role {
                    Some(role) => write!(f, "{link} [{role}]"),
                    None => write!(f, "{link}"),
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum RecordRefConstructionError {
    #[error("invalid record id: {0}")]
    InvalidId(#[from] IdParseError),
    #[error("invalid record kind: {0}")]
    InvalidKind(#[from] RecordKindParseError),
    #[error("invalid link: {0}")]
    InvalidLink(#[from] oneiros_link::LinkError),
}

impl<A, B> TryFrom<(A, B, Option<String>)> for IdentifiedRef
where
    A: AsRef<str>,
    B: AsRef<str>,
{
    type Error = RecordRefConstructionError;

    fn try_from((id, kind, role): (A, B, Option<String>)) -> Result<Self, Self::Error> {
        Ok(IdentifiedRef {
            id: id.as_ref().parse()?,
            kind: kind.as_ref().parse()?,
            role: role.map(Label::new),
        })
    }
}

impl IdentifiedRef {
    pub fn construct_from_db(
        row: impl TryInto<Self, Error = RecordRefConstructionError>,
    ) -> Result<RecordRef, RecordRefConstructionError> {
        Ok(RecordRef::Identified(row.try_into()?))
    }
}

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
    fn identified_ref_roundtrips() {
        let r = RecordRef::identified(Id::new(), RecordKind::Cognition, Some(Label::new("origin")));
        let json = serde_json::to_string(&r).unwrap();
        let deserialized: RecordRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r, deserialized);
    }

    #[test]
    fn linked_ref_roundtrips() {
        let link = Link::new(&("cognition", "some-content")).unwrap();
        let r = RecordRef::linked(link, Some(Label::new("origin")));
        let json = serde_json::to_string(&r).unwrap();
        let deserialized: RecordRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r, deserialized);
    }

    #[test]
    fn identified_ref_deserializes_from_legacy_json() {
        let id = Id::new();
        let json = format!(r#"{{"id":"{}","kind":"cognition","role":"origin"}}"#, id);
        let r: RecordRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r.id(), Some(&id));
        assert_eq!(r.kind(), Some(&RecordKind::Cognition));
        assert_eq!(r.role().map(|l| l.as_str()), Some("origin"));
        assert!(r.link().is_none());
    }

    #[test]
    fn linked_ref_deserializes_from_link_json() {
        let link = Link::new(&("memory", "some-content")).unwrap();
        let json = format!(r#"{{"link":"{}","role":"origin"}}"#, link);
        let r: RecordRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r.link(), Some(&link));
        assert_eq!(r.role().map(|l| l.as_str()), Some("origin"));
        assert!(r.id().is_none());
        assert!(r.kind().is_none());
    }

    #[test]
    fn mixed_refs_in_vec() {
        let id = Id::new();
        let link = Link::new(&("cognition", "a-thought")).unwrap();
        let refs = vec![
            RecordRef::identified(id, RecordKind::Cognition, Some(Label::new("origin"))),
            RecordRef::linked(link.clone(), Some(Label::new("echo"))),
        ];
        let json = serde_json::to_string(&refs).unwrap();
        let deserialized: Vec<RecordRef> = serde_json::from_str(&json).unwrap();
        assert_eq!(refs, deserialized);
    }

    #[test]
    fn linked_ref_without_role() {
        let link = Link::new(&("experience", "thread")).unwrap();
        let r = RecordRef::linked(link.clone(), None);
        let json = serde_json::to_string(&r).unwrap();
        let deserialized: RecordRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r, deserialized);
        assert!(deserialized.role().is_none());
    }

    #[test]
    fn display_identified() {
        let r = RecordRef::identified(Id::new(), RecordKind::Memory, Some(Label::new("origin")));
        let display = r.to_string();
        assert!(display.contains("memory"));
        assert!(display.contains("[origin]"));
    }

    #[test]
    fn display_linked() {
        let link = Link::new(&("cognition", "thought")).unwrap();
        let r = RecordRef::linked(link.clone(), Some(Label::new("echo")));
        let display = r.to_string();
        assert!(display.contains(&link.to_string()));
        assert!(display.contains("[echo]"));
    }
}
