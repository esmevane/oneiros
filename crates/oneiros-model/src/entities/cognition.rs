use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum CognitionConstructionError {
    #[error("invalid cognition id: {0}")]
    InvalidId(IdParseError),
    #[error("invalid agent id: {0}")]
    InvalidAgentId(IdParseError),
    #[error("invalid created_at timestamp: {0}")]
    InvalidCreatedAt(#[from] TimestampParseError),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cognition {
    pub id: CognitionId,
    pub agent_id: AgentId,
    pub texture: TextureName,
    pub content: Content,
    pub created_at: Timestamp,
}

impl Cognition {
    pub fn create(agent_id: AgentId, texture: TextureName, content: Content) -> Self {
        Self {
            id: CognitionId::from(Id::new()),
            agent_id,
            texture,
            content,
            created_at: Timestamp::now(),
        }
    }

    pub fn ref_token(&self) -> RefToken {
        RefToken::new(Ref::cognition(self.id))
    }

    pub fn as_detail(&self) -> String {
        let lines = [
            format!("  Texture: {}", self.texture),
            format!("  Content: {}", self.content),
            format!("  Created: {}", self.created_at),
        ];

        lines.join("\n")
    }

    pub fn as_table_row(&self) -> String {
        let texture = format!("{}", self.texture);
        let content = self.content.as_str();
        let truncated = if content.len() > 80 {
            let end = content.floor_char_boundary(80);
            format!("{}...", &content[..end])
        } else {
            content.to_string()
        };

        format!("{texture:<12} {truncated}")
    }

    pub fn construct_from_db(
        (id, agent_id, texture, content, created_at): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
    ) -> Result<Self, CognitionConstructionError> {
        Ok(Cognition {
            id: id
                .as_ref()
                .parse()
                .map_err(CognitionConstructionError::InvalidId)?,
            agent_id: agent_id
                .as_ref()
                .parse()
                .map_err(CognitionConstructionError::InvalidAgentId)?,
            texture: TextureName::new(texture),
            content: Content::new(content),
            created_at: Timestamp::parse_str(created_at)?,
        })
    }
}

impl core::fmt::Display for Cognition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.ref_token(), self.as_table_row())
    }
}

domain_id!(CognitionId);
