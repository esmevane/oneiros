use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// The full cognitive context for an agent — assembled by dream/introspect.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CognitiveContext {
    pub agent: Agent,
    pub cognitions: Vec<Cognition>,
    pub memories: Vec<Memory>,
    pub experiences: Vec<Experience>,
}

/// A lifecycle event marker — records that a lifecycle operation occurred.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LifecycleMarker {
    pub agent: String,
    pub operation: String,
    pub created_at: String,
}
