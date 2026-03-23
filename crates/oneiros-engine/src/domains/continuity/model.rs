use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A continuity event marker — records that a continuity operation occurred.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContinuityMarker {
    pub agent: AgentName,
    pub operation: Label,
    pub created_at: Timestamp,
}
