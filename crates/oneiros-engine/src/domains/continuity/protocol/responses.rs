use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
pub enum ContinuityResponse {
    Emerged(CognitiveContext),
    Waking(CognitiveContext),
    Dreaming(CognitiveContext),
    Introspecting(CognitiveContext),
    Reflecting(CognitiveContext),
    Sleeping(CognitiveContext),
    Receded(AgentName),
    Status(CognitiveContext),
    Guidebook(CognitiveContext),
}
