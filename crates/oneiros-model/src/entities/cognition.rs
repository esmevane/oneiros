use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cognition {
    pub id: CognitionId,
    pub agent_id: AgentId,
    pub texture: TextureName,
    pub content: Content,
    pub created_at: DateTime<Utc>,
}

impl Cognition {
    fn as_table_row(&self) -> String {
        let short_id = &self.id.to_string()[..8];
        let texture = format!("{}", self.texture);
        let content = self.content.as_str();
        let truncated = if content.len() > 80 {
            let end = content.floor_char_boundary(80);
            format!("{}...", &content[..end])
        } else {
            content.to_string()
        };

        format!("{short_id}  {texture:<12} {truncated}")
    }
}

impl core::fmt::Display for Cognition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_table_row())
    }
}

domain_id!(CognitionId);
