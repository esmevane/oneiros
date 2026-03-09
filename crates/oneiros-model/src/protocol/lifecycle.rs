use serde::{Deserialize, Serialize};

use super::agent::{CreateAgentRequest, SelectAgentByName};
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DreamCompleteEvent {
    pub agent: Agent,
}

/// Configuration for dream assembly — controls BFS traversal depth,
/// size caps, and memory level filtering.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DreamConfig {
    /// Number of recent cognitions and experiences to include
    /// in the orientation window.
    pub recent_window: usize,
    /// Maximum BFS traversal depth from the seed set.
    /// None means unlimited.
    pub dream_depth: Option<usize>,
    /// Maximum number of cognitions in the dream.
    /// None means unlimited.
    pub cognition_size: Option<usize>,
    /// Minimum memory level to include (log-level semantics).
    /// Core memories are always included regardless of this setting.
    /// None means include all levels.
    pub recollection_level: Option<LevelName>,
    /// Maximum number of non-core memories in the dream.
    /// None means unlimited.
    pub recollection_size: Option<usize>,
    /// Maximum number of experiences in the dream.
    /// None means unlimited.
    pub experience_size: Option<usize>,
}

impl Default for DreamConfig {
    fn default() -> Self {
        Self {
            recent_window: 5,
            dream_depth: Some(1),
            cognition_size: Some(20),
            recollection_level: Some(LevelName::new("project")),
            recollection_size: Some(30),
            experience_size: Some(10),
        }
    }
}

// ── Request types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WakeRequest {
    pub agent: AgentName,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SleepRequest {
    pub agent: AgentName,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct RecedeRequest {
    pub agent: AgentName,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DreamRequest {
    pub agent: AgentName,
    #[serde(default)]
    pub config: DreamConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct IntrospectRequest {
    pub agent: AgentName,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ReflectRequest {
    pub agent: AgentName,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SenseRequest {
    pub agent: AgentName,
}

// ── Event enums ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LifecycleEvents {
    Woke(SelectAgentByName),
    Slept(SelectAgentByName),
    Emerged(SelectAgentByName),
    Receded(SelectAgentByName),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum DreamingEvents {
    DreamBegun(SelectAgentByName),
    DreamComplete(DreamCompleteEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum IntrospectingEvents {
    IntrospectionBegun(SelectAgentByName),
    IntrospectionComplete(SelectAgentByName),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ReflectingEvents {
    ReflectionBegun(SelectAgentByName),
    ReflectionComplete(SelectAgentByName),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SenseEvents {
    Sensed(SelectAgentByName),
}

// ── Request enums ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LifecycleRequests {
    Wake(WakeRequest),
    Sleep(SleepRequest),
    Emerge(CreateAgentRequest),
    Recede(RecedeRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum DreamingRequests {
    Dream(DreamRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum IntrospectingRequests {
    Introspect(IntrospectRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ReflectingRequests {
    Reflect(ReflectRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SenseRequests {
    Sense(SenseRequest),
}

// ── Response enums ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum LifecycleResponses {
    Woke(Box<DreamContext>),
    Slept(Agent),
    Emerged(Agent),
    Receded,
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum DreamingResponses {
    DreamComplete(Box<DreamContext>),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum IntrospectingResponses {
    IntrospectionComplete(Agent),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ReflectingResponses {
    ReflectionComplete(Agent),
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum SenseResponses {
    Sensed(Agent),
}
