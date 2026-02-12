use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cognition {
    pub id: CognitionId,
    pub agent_id: AgentId,
    pub texture: TextureName,
    pub content: Content,
    pub created_at: DateTime<Utc>,
}

domain_id!(CognitionId);
