use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub enum ExperienceConstructionError {
    #[error("invalid experience id: {0}")]
    InvalidId(IdParseError),
    #[error("invalid agent id: {0}")]
    InvalidAgentId(IdParseError),
    #[error("invalid created_at timestamp: {0}")]
    InvalidCreatedAt(#[from] TimestampParseError),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Experience {
    pub id: ExperienceId,
    pub agent_id: AgentId,
    pub sensation: SensationName,
    pub description: Description,
    pub created_at: Timestamp,
}

impl Experience {
    pub fn create(agent_id: AgentId, sensation: SensationName, description: Description) -> Self {
        Self {
            id: ExperienceId::from(Id::new()),
            agent_id,
            sensation,
            description,
            created_at: Timestamp::now(),
        }
    }

    pub fn ref_token(&self) -> RefToken {
        RefToken::new(Ref::experience(self.id))
    }

    pub fn as_table_row(&self) -> String {
        let sensation = format!("{}", self.sensation);
        let desc = self.description.as_str();
        let truncated = if desc.len() > 80 {
            let end = desc.floor_char_boundary(80);
            format!("{}...", &desc[..end])
        } else {
            desc.to_string()
        };

        format!("{sensation:<12} {truncated}")
    }

    pub fn as_detail(&self) -> String {
        let lines = [
            format!("  Sensation: {}", self.sensation),
            format!("  Description: {}", self.description),
        ];

        lines.join("\n")
    }

    pub fn construct_from_db(
        (id, agent_id, sensation, description, created_at): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
    ) -> Result<Experience, ExperienceConstructionError> {
        Ok(Experience {
            id: id
                .as_ref()
                .parse()
                .map_err(ExperienceConstructionError::InvalidId)?,
            agent_id: agent_id
                .as_ref()
                .parse()
                .map_err(ExperienceConstructionError::InvalidAgentId)?,
            sensation: SensationName::new(sensation),
            description: Description::new(description),
            created_at: Timestamp::parse_str(created_at)?,
        })
    }
}

impl core::fmt::Display for Experience {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.ref_token(), self.as_table_row())
    }
}

domain_id!(ExperienceId);
