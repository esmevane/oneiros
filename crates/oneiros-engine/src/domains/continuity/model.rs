use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::*;

/// A continuity event marker — records that a continuity operation occurred.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub(crate) struct ContinuityMarker {
    pub(crate) agent: AgentName,
    pub(crate) operation: Label,
    pub(crate) created_at: Timestamp,
}
