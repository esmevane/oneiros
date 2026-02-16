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

domain_id!(ExperienceId);
