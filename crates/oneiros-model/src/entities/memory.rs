use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Memory {
    pub id: MemoryId,
    pub agent_id: AgentId,
    pub level: LevelName,
    pub content: Content,
    pub created_at: DateTime<Utc>,
}

domain_id!(MemoryId);
