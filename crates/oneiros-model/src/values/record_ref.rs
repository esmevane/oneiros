use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordRef {
    pub id: Id,
    pub kind: RecordKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Label>,
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

impl core::str::FromStr for RecordKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cognition" => Ok(RecordKind::Cognition),
            "memory" => Ok(RecordKind::Memory),
            "experience" => Ok(RecordKind::Experience),
            "storage" => Ok(RecordKind::Storage),
            other => Err(format!("unknown record kind: {other}")),
        }
    }
}
