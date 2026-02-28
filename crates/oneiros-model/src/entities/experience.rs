use serde::{Deserialize, Serialize};

use crate::*;

/// A reference from an experience to any entity, with an optional role label.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExperienceRef {
    pub entity: Ref,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Label>,
}

impl ExperienceRef {
    pub fn new(entity: Ref, role: Option<Label>) -> Self {
        Self { entity, role }
    }
}

impl core::fmt::Display for ExperienceRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let token = RefToken::new(self.entity.clone());
        match &self.role {
            Some(role) => write!(f, "{token} [{role}]"),
            None => write!(f, "{token}"),
        }
    }
}

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
    pub refs: Vec<ExperienceRef>,
    pub created_at: Timestamp,
}

impl Experience {
    pub fn create(
        agent_id: AgentId,
        sensation: SensationName,
        description: Description,
        refs: Vec<ExperienceRef>,
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
        refs: Vec<ExperienceRef>,
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
        write!(f, "{} {}", self.ref_token(), self.as_table_row())
    }
}

domain_id!(ExperienceId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experience_ref_display_with_role() {
        let r = ExperienceRef::new(
            Ref::cognition(CognitionId::new()),
            Some(Label::new("origin")),
        );
        let display = r.to_string();
        assert!(display.contains("[origin]"));
    }

    #[test]
    fn experience_ref_display_without_role() {
        let r = ExperienceRef::new(Ref::memory(MemoryId::new()), None);
        let display = r.to_string();
        assert!(!display.contains('['));
    }

    #[test]
    fn experience_ref_serde_roundtrip() {
        let r = ExperienceRef::new(Ref::cognition(CognitionId::new()), Some(Label::new("echo")));
        let json = serde_json::to_string(&r).unwrap();
        let deserialized: ExperienceRef = serde_json::from_str(&json).unwrap();
        assert_eq!(r, deserialized);
    }

    #[test]
    fn experience_ref_without_role_omits_field() {
        let r = ExperienceRef::new(Ref::agent(AgentId::new()), None);
        let json = serde_json::to_string(&r).unwrap();
        assert!(!json.contains("role"));
    }
}
