use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::*;

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

domain_id!(MemoryId);
