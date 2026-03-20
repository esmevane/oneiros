use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// The full cognitive context for an agent — assembled by dream/introspect.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CognitiveContext {
    pub agent: Agent,
    #[serde(default)]
    pub cognitions: Vec<Cognition>,
    #[serde(default)]
    pub memories: Vec<Memory>,
    #[serde(default)]
    pub experiences: Vec<Experience>,
}

/// A lifecycle event marker — records that a lifecycle operation occurred.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LifecycleMarker {
    pub agent: AgentName,
    pub operation: Label,
    pub created_at: Timestamp,
}
