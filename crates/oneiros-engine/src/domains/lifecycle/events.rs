use serde::{Deserialize, Serialize};

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
    pub agent: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensedEvent {
    pub agent: String,
    pub content: String,
    pub created_at: String,
}
