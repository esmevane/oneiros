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
    InvalidCreatedAt(#[from] TimestampConstructionFailure),
}

pub type ExperienceRecord = Record<ExperienceId, HasDescription<HasRefs<Experience>>>;

impl ExperienceRecord {
    pub fn init(
        description: impl Into<Description>,
        refs: Vec<RecordRef>,
        experience: Experience,
    ) -> Self {
        Record::create(HasDescription::new(
            description.into(),
            HasRefs::new(refs, experience),
        ))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Experience {
    pub agent_id: AgentId,
    pub sensation: SensationName,
}

impl Experience {
    pub fn as_table_row(&self, description: &Description, refs: &[RecordRef]) -> String {
        let sensation = format!("{}", self.sensation);
        let desc = description.as_str();
        let truncated = if desc.len() > 80 {
            let end = desc.floor_char_boundary(80);
            format!("{}...", &desc[..end])
        } else {
            desc.to_string()
        };
        let ref_count = refs.len();

        format!("{sensation:<12} {truncated} ({ref_count} refs)")
    }

    pub fn as_detail(&self, description: &Description, refs: &[RecordRef]) -> String {
        let mut lines = vec![
            format!("  Sensation: {}", self.sensation),
            format!("  Description: {}", description),
        ];

        lines.push(format!("  Refs: ({})", refs.len()));
        for r in refs {
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
    ) -> Result<ExperienceRecord, ExperienceConstructionError> {
        let id: ExperienceId = id
            .as_ref()
            .parse()
            .map_err(ExperienceConstructionError::InvalidId)?;

        let agent_id: AgentId = agent_id
            .as_ref()
            .parse()
            .map_err(ExperienceConstructionError::InvalidAgentId)?;
        let experience = Experience {
            agent_id,
            sensation: SensationName::new(sensation),
        };

        Ok(Record::build(
            id,
            HasDescription::new(
                Description::new(description),
                HasRefs::new(refs, experience),
            ),
            created_at,
        )?)
    }
}

domain_link!(Experience => ExperienceLink);
domain_id!(ExperienceId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experience_identity() {
        let agent_id = AgentId::new();

        let primary = Experience {
            agent_id,
            sensation: SensationName::new("continues"),
        };

        let other = Experience {
            agent_id,
            sensation: SensationName::new("continues"),
        };

        assert_eq!(primary.as_link().unwrap(), other.as_link().unwrap());
    }
}
