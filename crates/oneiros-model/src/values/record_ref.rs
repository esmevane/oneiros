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
#[serde(untagged)]
pub enum RecordRef {
    Identified(IdentifiedRef),
}

impl RecordRef {
    pub fn identified(id: Id, kind: RecordKind, role: Option<Label>) -> Self {
        RecordRef::Identified(IdentifiedRef { id, kind, role })
    }

    pub fn id(&self) -> &Id {
        match self {
            RecordRef::Identified(r) => &r.id,
        }
    }

    pub fn kind(&self) -> &RecordKind {
        match self {
            RecordRef::Identified(r) => &r.kind,
        }
    }

    pub fn role(&self) -> Option<&Label> {
        match self {
            RecordRef::Identified(r) => r.role.as_ref(),
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
        }
    }
}

#[derive(Debug, Error)]
pub enum RecordRefConstructionError {
    #[error("invalid record id: {0}")]
    InvalidId(#[from] IdParseError),
    #[error("invalid record kind: {0}")]
    InvalidKind(#[from] RecordKindParseError),
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
