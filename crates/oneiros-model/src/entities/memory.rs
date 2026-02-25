use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum MemoryConstructionError {
    #[error("invalid memory id: {0}")]
    InvalidId(IdParseError),
    #[error("invalid agent id: {0}")]
    InvalidAgentId(IdParseError),
    #[error("invalid created_at timestamp: {0}")]
    InvalidCreatedAt(#[from] TimestampConstructionFailure),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Memory {
    pub agent_id: AgentId,
    pub level: LevelName,
    pub content: Content,
}

impl Memory {
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
    ) -> Result<Record<MemoryId, Self>, MemoryConstructionError> {
        let id: MemoryId = id
            .as_ref()
            .parse()
            .map_err(MemoryConstructionError::InvalidId)?;

        let agent_id: AgentId = agent_id
            .as_ref()
            .parse()
            .map_err(MemoryConstructionError::InvalidAgentId)?;
        let memory = Memory {
            agent_id,
            level: LevelName::new(level),
            content: Content::new(content),
        };

        Ok(Record::build(id, memory, created_at)?)
    }
}

impl core::fmt::Display for Memory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_table_row())
    }
}

domain_link!(Memory => MemoryLink);
domain_id!(MemoryId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_identity() {
        let agent_id = AgentId::new();
        let primary = Memory {
            agent_id,
            level: LevelName::new("project"),
            content: Content::new("oneiros is an identity substrate"),
        };

        let other = Memory {
            agent_id,
            level: LevelName::new("project"),
            content: Content::new("oneiros is an identity substrate"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
