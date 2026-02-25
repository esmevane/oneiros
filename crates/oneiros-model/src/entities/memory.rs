use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum MemoryConstructionError {
    #[error("invalid memory id: {0}")]
    InvalidId(IdParseError),
    #[error("invalid agent id: {0}")]
    InvalidAgentId(IdParseError),
    #[error("invalid created_at timestamp: {0}")]
    InvalidCreatedAt(#[from] TimestampParseError),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Memory {
    pub id: MemoryId,
    pub agent_id: AgentId,
    pub level: LevelName,
    pub content: Content,
    pub created_at: Timestamp,
}

impl Memory {
    pub fn create(agent_id: AgentId, level: LevelName, content: Content) -> Self {
        Self {
            id: MemoryId::from(Id::new()),
            agent_id,
            level,
            content,
            created_at: Timestamp::now(),
        }
    }

    pub fn as_table_row(&self) -> String {
        let level = format!("{}", self.level);
        let content = self.content.as_str();
        let truncated = if content.len() > 80 {
            let end = content.floor_char_boundary(80);
            format!("{}...", &content[..end])
        } else {
            content.to_string()
        };

        format!("{level:<12} {truncated}")
    }

    pub fn construct_from_db(
        (id, agent_id, level, content, created_at): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
    ) -> Result<Self, MemoryConstructionError> {
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
            created_at: Timestamp::parse_str(created_at)?,
        })
    }
}

impl core::fmt::Display for Memory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let id = self.id.to_string();
        let prefix = if id.len() >= 8 { &id[..8] } else { &id };
        write!(f, "{prefix:<10}{}", self.as_table_row())
    }
}

domain_id!(MemoryId);
