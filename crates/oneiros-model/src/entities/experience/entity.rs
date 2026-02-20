use chrono::{DateTime, Utc};
use oneiros_link::*;
use serde::{Deserialize, Serialize};

use crate::*;

use super::ExperienceConstructionError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Experience {
    pub agent_id: AgentId,
    pub sensation: SensationName,
    pub description: Content,
    pub refs: Vec<RecordRef>,
    pub created_at: DateTime<Utc>,
}

impl Experience {
    pub fn as_table_row(&self) -> String {
        let sensation = format!("{}", self.sensation);
        let description = self.description.as_str();
        let truncated = if description.len() > 80 {
            let end = description.floor_char_boundary(80);
            format!("{}...", &description[..end])
        } else {
            description.to_string()
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

        lines.push(format!("  Created: {}", self.created_at));

        lines.join("\n")
    }
}

impl core::fmt::Display for Experience {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_table_row())
    }
}

impl Experience {
    pub fn construct_from_db(
        (id, agent_id, sensation, description, created_at): (
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
            impl AsRef<str>,
        ),
        refs: Vec<RecordRef>,
    ) -> Result<Identity<ExperienceId, Self>, ExperienceConstructionError> {
        let id: ExperienceId = id
            .as_ref()
            .parse()
            .map_err(ExperienceConstructionError::InvalidId)?;
        let experience = Experience {
            agent_id: agent_id
                .as_ref()
                .parse()
                .map_err(ExperienceConstructionError::InvalidAgentId)?,
            sensation: SensationName::new(sensation),
            description: Content::new(description),
            refs,
            created_at: created_at
                .as_ref()
                .parse::<DateTime<Utc>>()
                .map_err(ExperienceConstructionError::InvalidCreatedAt)?,
        };
        Ok(Identity::new(id, experience))
    }
}

impl Addressable for Experience {
    fn address_label() -> &'static str {
        "experience"
    }

    fn link(&self) -> Result<Link, LinkError> {
        // The experience is the identity: what sensation and description.
        // Agent, timestamp, and refs are context or mutable.
        Link::new(&(Self::address_label(), &self.sensation, &self.description))
    }
}

domain_id!(ExperienceId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experience_identity_same_sensation_and_description() {
        let primary = Experience {
            agent_id: AgentId::new(),
            sensation: SensationName::new("continues"),
            description: Content::new("a thread"),
            refs: vec![],
            created_at: Utc::now(),
        };

        // Different agent, timestamp, and refs — same link (sensation + description match)
        let other = Experience {
            agent_id: AgentId::new(),
            sensation: SensationName::new("continues"),
            description: Content::new("a thread"),
            refs: vec![RecordRef::identified(
                Id::new(),
                RecordKind::Cognition,
                None,
            )],
            created_at: Utc::now(),
        };

        assert_eq!(primary.link().unwrap(), other.link().unwrap());
    }

    #[test]
    fn experience_identity_different_description() {
        let primary = Experience {
            agent_id: AgentId::new(),
            sensation: SensationName::new("continues"),
            description: Content::new("a thread"),
            refs: vec![],
            created_at: Utc::now(),
        };

        // Same sensation, different description — different link
        let other = Experience {
            agent_id: AgentId::new(),
            sensation: SensationName::new("continues"),
            description: Content::new("a different thread"),
            refs: vec![],
            created_at: Utc::now(),
        };

        assert_ne!(primary.link().unwrap(), other.link().unwrap());
    }
}
