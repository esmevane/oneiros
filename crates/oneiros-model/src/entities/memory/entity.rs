use chrono::{DateTime, Utc};
use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

use super::MemoryConstructionError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Memory {
    pub agent_id: AgentId,
    pub level: LevelName,
    pub content: Content,
    pub created_at: DateTime<Utc>,
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
}

impl core::fmt::Display for Memory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_table_row())
    }
}

impl Memory {
    pub fn construct_from_db(
        (id, agent_id, level, content, created_at): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
    ) -> Result<Identity<MemoryId, Self>, MemoryConstructionError> {
        let id: MemoryId = id
            .as_ref()
            .parse()
            .map_err(MemoryConstructionError::InvalidId)?;
        let memory = Memory {
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
        };
        Ok(Identity::new(id, memory))
    }
}

impl Addressable for Memory {
    fn address_label() -> &'static str {
        "memory"
    }

    fn link(&self) -> Result<Link, LinkError> {
        // The memory is the identity: what level and what content.
        // Agent and timestamp are context.
        Link::new(&(Self::address_label(), &self.level, &self.content))
    }
}

domain_id!(MemoryId);
oneiros_link::domain_link!(MemoryLink, "memory");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_identity() {
        let primary = Memory {
            agent_id: AgentId::new(),
            level: LevelName::new("project"),
            content: Content::new("oneiros is an identity substrate"),
            created_at: Utc::now(),
        };

        // Different agent and timestamp — same link
        let other = Memory {
            agent_id: AgentId::new(),
            level: LevelName::new("project"),
            content: Content::new("oneiros is an identity substrate"),
            created_at: Utc::now(),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }
}
