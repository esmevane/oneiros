use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::*;

use super::CognitionConstructionError;

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

impl<A, B, C, D, E> TryFrom<(A, B, C, D, E)> for Cognition
where
    A: AsRef<str>,
    B: AsRef<str>,
    C: AsRef<str>,
    D: AsRef<str>,
    E: AsRef<str>,
{
    type Error = CognitionConstructionError;

    fn try_from(
        (id, agent_id, texture, content, created_at): (A, B, C, D, E),
    ) -> Result<Self, Self::Error> {
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
            created_at: created_at
                .as_ref()
                .parse::<DateTime<Utc>>()
                .map_err(CognitionConstructionError::InvalidCreatedAt)?,
        })
    }
}

impl Cognition {
    pub fn construct_from_db(
        row: impl TryInto<Self, Error = CognitionConstructionError>,
    ) -> Result<Self, CognitionConstructionError> {
        row.try_into()
    }
}

domain_id!(CognitionId);
