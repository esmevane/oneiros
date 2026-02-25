use oneiros_link::*;
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
    #[serde(default)]
    pub refs: Vec<RecordRef>,
    pub created_at: Timestamp,
}

impl Experience {
    pub fn create(
        agent_id: AgentId,
        sensation: SensationName,
        description: Description,
        refs: Vec<RecordRef>,
    ) -> Self {
        Self {
            id: ExperienceId::from(Id::new()),
            agent_id,
            sensation,
            description,
            refs,
            created_at: Timestamp::now(),
        }
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
        let ref_count = self.refs.len();

        format!("{sensation:<12} {truncated} ({ref_count} refs)")
    }

    pub fn as_detail(&self) -> String {
        let mut lines = vec![
            format!("  Sensation: {}", self.sensation),
            format!("  Description: {}", self.description),
        ];

        lines.push(format!("  Refs: ({})", self.refs.len()));
        for r in &self.refs {
            lines.push(format!("    {r}"));
        }

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
        refs: Vec<RecordRef>,
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
            refs,
            created_at: Timestamp::parse_str(created_at)?,
        })
    }
}

impl core::fmt::Display for Experience {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let prefix = &self.id.to_string()[..8];
        write!(f, "{prefix} {}", self.as_table_row())
    }
}

domain_link!(Experience => ExperienceLink);
domain_id!(ExperienceId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experience_same_fields_same_link() {
        let experience = Experience::create(
            AgentId::new(),
            SensationName::new("continues"),
            Description::new("desc"),
            vec![],
        );

        let clone = experience.clone();

        assert_eq!(experience.as_link().unwrap(), clone.as_link().unwrap());
    }
}
