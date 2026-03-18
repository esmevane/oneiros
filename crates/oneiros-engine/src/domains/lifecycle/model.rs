use serde::{Deserialize, Serialize};

use crate::domains::agent::model::Agent;
use crate::domains::cognition::model::Cognition;
use crate::domains::memory::model::Memory;
use crate::domains::experience::model::Experience;

/// The full cognitive context for an agent — assembled by dream/introspect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveContext {
    pub agent: Agent,
    pub cognitions: Vec<Cognition>,
    pub memories: Vec<Memory>,
    pub experiences: Vec<Experience>,
}

/// A lifecycle event marker — records that a lifecycle operation occurred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleMarker {
    pub agent: String,
    pub operation: String,
    pub created_at: String,
}
