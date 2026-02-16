use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Experience {
    pub id: ExperienceId,
    pub agent_id: AgentId,
    pub sensation: SensationName,
    pub description: Content,
    pub refs: Vec<RecordRef>,
    pub created_at: DateTime<Utc>,
}

impl Experience {
    fn as_table_row(&self) -> String {
        let short_id = &self.id.to_string()[..8];
        let sensation = format!("{}", self.sensation);
        let description = self.description.as_str();
        let truncated = if description.len() > 80 {
            let end = description.floor_char_boundary(80);
            format!("{}...", &description[..end])
        } else {
            description.to_string()
        };
        let ref_count = self.refs.len();

        format!("{short_id}  {sensation:<12} {truncated} ({ref_count} refs)")
    }

    pub fn as_detail(&self) -> String {
        let mut lines = vec![
            format!("Experience {}", self.id),
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

domain_id!(ExperienceId);
