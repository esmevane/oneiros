use chrono::{DateTime, Utc};
use oneiros_link::*;
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

impl Addressable for Cognition {
    fn address_label() -> &'static str {
        "cognition"
    }

    fn link(&self) -> Result<Link, LinkError> {
        // Content-addressable: the thought is the identity, regardless of
        // who said it or when. Agent and timestamp are context, not identity.
        Link::new(&(Self::address_label(), &self.texture, &self.content))
    }
}

domain_id!(CognitionId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cognition_identity() {
        let primary = Cognition {
            id: CognitionId::new(),
            agent_id: AgentId::new(),
            texture: TextureName::new("working"),
            content: Content::new("thinking about links"),
            created_at: Utc::now(),
        };

        // Different agent, different id, different timestamp — same link
        let other = Cognition {
            id: CognitionId::new(),
            agent_id: AgentId::new(),
            texture: TextureName::new("working"),
            content: Content::new("thinking about links"),
            created_at: Utc::now(),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }

    #[test]
    fn cognition_different_content_different_link() {
        let primary = Cognition {
            id: CognitionId::new(),
            agent_id: AgentId::new(),
            texture: TextureName::new("working"),
            content: Content::new("first thought"),
            created_at: Utc::now(),
        };

        let other = Cognition {
            id: CognitionId::new(),
            agent_id: AgentId::new(),
            texture: TextureName::new("working"),
            content: Content::new("second thought"),
            created_at: Utc::now(),
        };

        assert_ne!(primary.link().unwrap(), other.link().unwrap());
    }
}
