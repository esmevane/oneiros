use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LifecycleEvents {
    Dreamed(LifecycleEvent),
    Introspected(LifecycleEvent),
    Reflected(LifecycleEvent),
    Sensed(SensedEvent),
    Slept(LifecycleEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleEvent {
    pub agent: AgentName,
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensedEvent {
    pub agent: AgentName,
    pub content: Content,
    pub created_at: Timestamp,
}
