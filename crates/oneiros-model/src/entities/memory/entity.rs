use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::*;

use super::MemoryConstructionError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Memory {
    pub id: MemoryId,
    pub agent_id: AgentId,
    pub level: LevelName,
    pub content: Content,
    pub created_at: DateTime<Utc>,
}

impl Memory {
    fn as_table_row(&self) -> String {
        let short_id = &self.id.to_string()[..8];
        let level = format!("{}", self.level);
        let content = self.content.as_str();
        let truncated = if content.len() > 80 {
            let end = content.floor_char_boundary(80);
            format!("{}...", &content[..end])
        } else {
            content.to_string()
        };

        format!("{short_id}  {level:<12} {truncated}")
    }
}

impl core::fmt::Display for Memory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_table_row())
    }
}

impl<A, B, C, D, E> TryFrom<(A, B, C, D, E)> for Memory
where
    A: AsRef<str>,
    B: AsRef<str>,
    C: AsRef<str>,
    D: AsRef<str>,
    E: AsRef<str>,
{
    type Error = MemoryConstructionError;

    fn try_from(
        (id, agent_id, level, content, created_at): (A, B, C, D, E),
    ) -> Result<Self, Self::Error> {
        Ok(Memory {
            id: id
                .as_ref()
                .parse()
                .map_err(MemoryConstructionError::InvalidId)?,
            agent_id: agent_id
                .as_ref()
                .parse()
                .map_err(MemoryConstructionError::InvalidAgentId)?,
            level: LevelName::new(level),
            content: Content::new(content),
            created_at: created_at
                .as_ref()
                .parse::<DateTime<Utc>>()
                .map_err(MemoryConstructionError::InvalidCreatedAt)?,
        })
    }
}

impl Memory {
    pub fn construct_from_db(
        row: impl TryInto<Self, Error = MemoryConstructionError>,
    ) -> Result<Self, MemoryConstructionError> {
        row.try_into()
    }
}

domain_id!(MemoryId);
