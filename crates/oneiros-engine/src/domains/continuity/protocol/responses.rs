use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ContinuityResponse {
    Emerged(DreamContext),
    Waking(DreamContext),
    Dreaming(DreamContext),
    Introspecting(DreamContext),
    Reflecting(DreamContext),
    Sleeping(DreamContext),
    Receded(AgentName),
    Status(DreamContext),
    Guidebook(DreamContext),
}
