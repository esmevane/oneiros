use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ContinuityEvents {
    Dreamed(ContinuityEvent),
    Introspected(ContinuityEvent),
    Reflected(ContinuityEvent),
    Sensed(SensedEvent),
    Slept(ContinuityEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuityEvent {
    pub agent: AgentName,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensedEvent {
    pub agent: AgentName,
    pub content: Content,
    pub created_at: Timestamp,
}
