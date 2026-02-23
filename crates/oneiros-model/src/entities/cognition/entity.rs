use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

use super::CognitionConstructionError;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cognition {
    pub agent_id: Key<AgentId, AgentLink>,
    pub texture: TextureName,
    pub content: Content,
}

impl Cognition {
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
    ) -> Result<Record<CognitionId, Self>, CognitionConstructionError> {
        let id: CognitionId = id
            .as_ref()
            .parse()
            .map_err(CognitionConstructionError::InvalidId)?;

        let agent_id: Key<AgentId, AgentLink> = agent_id.as_ref().parse()?;

        let cognition = Cognition {
            agent_id,
            texture: TextureName::new(texture),
            content: Content::new(content),
        };

        Ok(Record::build(id, cognition, created_at)?)
    }
}

impl core::fmt::Display for Cognition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_table_row())
    }
}

domain_link!(Cognition => CognitionLink);
domain_id!(CognitionId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cognition_identity() {
        let agent_id = Key::Id(AgentId::new());

        let primary = Cognition {
            agent_id: agent_id.clone(),
            texture: TextureName::new("working"),
            content: Content::new("thinking about links"),
        };

        let other = Cognition {
            agent_id,
            texture: TextureName::new("working"),
            content: Content::new("thinking about links"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }

    #[test]
    fn cognition_different_content_different_link() {
        let primary = Cognition {
            agent_id: Key::Id(AgentId::new()),
            texture: TextureName::new("working"),
            content: Content::new("first thought"),
        };

        let other = Cognition {
            agent_id: Key::Id(AgentId::new()),
            texture: TextureName::new("working"),
            content: Content::new("second thought"),
        };

        assert_ne!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
